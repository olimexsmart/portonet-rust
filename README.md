Learn Rust while refreshing a very useful project of mine. 

## GOALS V1
- [x] Rewrite all PHP functionality in Rust
- [x] Use SQLite
- [x] Do not modify frontend for the moment except for some API calls
- [x] Make it a docker
- [ ] Every API call is logged along with the parameters and IP
- [x] The door is opened through an API call configurable via ENV file
- [x] If system table is empty should be initialized 
- [x] Tables should be created if not existent
- [ ] Every API that checks master password should also be subject to system block to avoid brute forcing
## GOALS V2
- [v] Bump frontend to a recent Bootstrap version
- [ ] Implement Web AUTH for biometric login
- [ ] Delete unused buttons and related APIs
- [ ] Avoid modal, display box or a banner (nice libraries out there)

## Run with auto-reload
```
cargo watch -c -w src/ -x 'run'
```

## APIs
- `/add_key`
	- Inputs: `[masterPassword, newKey, duration]`
	- Verify master password validity and then insert new key with the specified duration
	- If the key is already present, duration should be updated instead (key duration refresh)
- `/get_counters`
	- Return: `{nOpenings: number, nErrors: number}`
- `/open_door`
	- Inputs: `[key, dryRun]`
	1. Check if system is locked: `lockedUntil > NOW()`
	2. Search key in table, manage if OK, not existent (wrong), expired or revoked
	3. Open door if OK, increment `nOpening`, reset `nAttempts`
	4. Else, increment `nErrors`, increment `nAttempts` only if key was not existent. This is part of the mechanism to avoid key brute forcing. 
	5. If `nAttempts` is above 10, lock system for 15 minutes: `SET lockedUntil = NOW() + 15 MIN` .  After 10 wrong attempts, system locks for 15 min at every subsequent wrong attempt.
- `/list_keys`
	- Inputs: `[masterPassword]`
	- Verify master password validity
	- Return an array of: 
```
		{
	        "ID": number,
	        "uKey": string,
	        "expDate": "2025-07-25 19:47:04",
	        "lastUsed": "2024-09-06 18:49:35",
	        "nUsed": number,
	        "revoked": boolean
		}
```
- `/getLog`
	- Inputs: `[masterPassword, limitN]`
	- Verify master password validity
	- Return an array `limitN` long of the last logs, as such:
```
    {
        "ID": "7228",
        "APIName": "logList",
        "dateRequest": "2024-09-07 17:52:40",
        "params": [<param1>, <param2>]
    },
```
- `/revoke_key`
	- Inputs: `[masterPassword, key]
	- Verify master password validity, set `revoked = 1` on specified key. Return positive response to not hint on key existence regardless if key exists or not.
- `/revoke_all_keys`
	- Inputs: `[masterPassword, key]
	- Verify master password validity, set `revoked = 1` on all access keys. Used to effectively lock the system.
- `/changeMasterPassword`
	- Inputs: `[oldMasterPassword, newMasterPassword]`
	- If old password is correct, change to the new one
	- At database creation the master password is set no null, call this method to initialize. Call with both parameter set to the same new password value.

## Docker

If running from MacOS, install `brew install colima`. Then start it with `colima start`. When done, `colima stop`.

### Build the image
This commands cross-compiles. Not necessary if building already from a Linux machine.
```bash
# Native
docker build -t portonet:latest .
# Cross compile
docker buildx build --platform linux/amd64 -t portonet:latest --load .
```

### Export the image to a tar file

```bash
docker save -o portonet.tar portonet:latest
```

### Deploy from the tar file

1. Copy `portonet.tar` to the target machine 
2. Create a `portonet-data` folder, copy there an existing `portonet.sqlite` file if available
3. Create an env file on the target machine, for example `/mnt/fastdisk/docker-data/portonet.env`, containing:

```env
DATABASE_URL=sqlite:/data/portonet.sqlite
MASTER_PASSWORD=your-master-password
BEARER_HOME_ASSISTANT=your-home-assistant-token
URL_HOME_ASSISTANT=https://your-home-assistant-host/api/services/button/press
ENTITY_HOME_ASSISTANT=button.apri_portone
```

4. Load the image: `docker load -i portonet.tar`
5. Start the container, passing the env file explicitly:

```bash
docker run -d \
  --name portonet \
  -p 8181:3000 \
  --env-file /mnt/fastdisk/docker-data/portonet.env \
  -v /mnt/fastdisk/docker-data/portonet-data:/data \
  --restart unless-stopped \
  portonet:latest
```

Change the 8181 value to the desired port. 

The env file stays on the target machine and does not go inside `portonet-data`; that directory is used for the SQLite database. Use the Docker database URL shown above so the database is stored in the mounted `/data` volume.
