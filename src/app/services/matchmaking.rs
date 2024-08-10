use std::{
	collections::{HashSet, VecDeque},
	mem,
	sync::OnceLock,
	time::Duration,
};

use rand::Rng;
use serde::Serialize;
use tokio::sync::{oneshot, Mutex, MutexGuard};

use crate::app::data::UserId;

use super::SqliteDatabase;

const MATCHMAKING_TIMEOUT_SECONDS: u64 = 10;
const PLAYERS_PER_MATCH: usize = 6;

static USED_PORTS: OnceLock<Mutex<HashSet<u16>>> = OnceLock::new();

#[derive(Default)]
pub struct MatchMaking(Mutex<State>);

#[derive(Clone, Debug, Serialize)]
pub struct MatchInfo {
	pub address: String,
	pub port: u16,
}

struct QueuedPlayer {
	id: UserId,
	sender: oneshot::Sender<MatchInfo>,
}

#[derive(Default)]
struct State {
	queue: VecDeque<QueuedPlayer>,
	queued_users: HashSet<UserId>,
}

async fn create_match(db: &SqliteDatabase) -> MatchInfo {
	let address = db.get_server_address();
	let used_ports = USED_PORTS
		.get_or_init(|| Mutex::new(HashSet::new()))
		.lock()
		.await;
	let mut rng = rand::thread_rng();
	let port = loop {
		let p: u16 = rng.gen_range(1024..u16::MAX);
		if !used_ports.contains(&p) {
			break p;
		}
	};

	mem::drop(used_ports);
	start_server(port);
	MatchInfo { address, port }
}

fn start_server(port: u16) {
	use std::process::Command;
	use std::thread;

	log::info!("starting server on port {port}");
	thread::spawn(move || {
		let mut used_ports = USED_PORTS.get().unwrap().blocking_lock();
		used_ports.insert(port);
		mem::drop(used_ports);

		match Command::new("./start-server.sh")
			.arg(port.to_string())
			.status()
		{
			Ok(v) => log::info!("server exited with {v}"),
			Err(e) => log::error!("error running server: {e}"),
		}

		let mut used_ports = USED_PORTS.get().unwrap().blocking_lock();
		used_ports.remove(&port);
		mem::drop(used_ports);
	});
}

impl MatchMaking {
	pub async fn queue_player(
		&self,
		user_id: UserId,
		db: &SqliteDatabase,
	) -> MatchInfo {
		let mut state = self.state().await;
		let receiver = if state.is_player_in_queue(&user_id) {
			state.update_receiver(&user_id)
		} else if state.queue.len() >= PLAYERS_PER_MATCH - 1 {
			let match_info = create_match(db).await;
			for _ in 0..PLAYERS_PER_MATCH - 1 {
				let player = state.dequeue_player().unwrap();
				let _ = player.sender.send(match_info.clone());
			}

			return match_info;
		} else {
			let (send, recv) = oneshot::channel();
			state.enqueue_player(QueuedPlayer {
				id: user_id,
				sender: send,
			});

			recv
		};

		mem::drop(state);
		let timeout = tokio::time::timeout(
			Duration::from_secs(MATCHMAKING_TIMEOUT_SECONDS),
			receiver,
		);

		if let Ok(v) = timeout.await {
			return v.unwrap();
		}

		// If we get timed out, create a match with all players currently in the queue
		let mut state = self.state().await;
		let match_info = create_match(db).await;
		assert!(state.queue.len() <= PLAYERS_PER_MATCH);

		while let Some(player) = state.dequeue_player() {
			// Ignore any potential send errors here, because
			// 1. our receiver is dropped, but the sender is still in the queue, so it will always error
			// 2. if any other queued players have had their receiver dropped, that's not a huge deal - it
			//    means that they're not connected to the server anymore anyways.
			let _ = player.sender.send(match_info.clone());
		}

		match_info
	}

	async fn state(&self) -> MutexGuard<'_, State> {
		self.0.lock().await
	}
}

impl State {
	fn enqueue_player(&mut self, player: QueuedPlayer) {
		self.queued_users.insert(player.id.clone());
		self.queue.push_back(player);
	}

	fn dequeue_player(&mut self) -> Option<QueuedPlayer> {
		let player = self.queue.pop_front()?;
		self.queued_users.remove(&player.id);
		Some(player)
	}

	fn is_player_in_queue(&self, id: &UserId) -> bool {
		self.queued_users.contains(id)
	}

	fn update_receiver(&mut self, id: &UserId) -> oneshot::Receiver<MatchInfo> {
		let player = self
			.queue
			.iter_mut()
			.find(|e| id == &e.id)
			.expect("player is not in the queue");

		let (send, recv) = oneshot::channel();
		player.sender = send;
		recv
	}
}
