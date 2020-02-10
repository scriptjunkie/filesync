# filesync

blake3 based file synchronization tool. Runas `filesync srcfile destfile` to copy/sync srcfile onto destfile.

For example, you may want to back up srcfile to destfile if destfile's path is on an external drive, or mounted SMB/NFS/SSHFS/etc. network drive.

No frills, confusing config options, any config options, backdoors, tracking, third parties, fluff, or other features.
You don't need to have software running on the remote side like rsync.

It will create a file with a .hashes extension to store hashes of each chunk of the destination file.
When you run it, it will compare hashes of the local file chunks to the destination hashfile contents, overwriting only the chunks that have changed.
It will extend the destination file if necessary but will not shrink it. Maybe I'll add that later if the people demand it.
