#!/usr/bin/sh
cargo build --release
if [ $? -ne 0 ]
then
	echo Build failed.
	exit 1
fi

scp target/release/gg root@31.220.60.205:/home/gameserver/gg
echo "All done. NOTE: This does NOT start the new gg instance!"
echo "IMPORTANT: Don't forget to apply any database schema changes!"
