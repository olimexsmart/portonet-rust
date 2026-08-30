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

## ENV setup

The `.env` file contains the following variables:

```
DATABASE_URL=sqlite://portonet.sqlite
MASTER_PASSWORD=your-master-password
BEARER_HOME_ASSISTANT=your-home-assistant-token
URL_HOME_ASSISTANT=https://your-home-assistant-host/api/services/light/toggle
ENTITY_HOME_ASSISTANT=light.your_entity
```

- `DATABASE_URL`: SQLite connection URL for the application database.
- `MASTER_PASSWORD`: Master password used to authorize protected API operations; it is not stored in the database.
- `BEARER_HOME_ASSISTANT`: Bearer token used to authenticate requests to Home Assistant.
- `URL_HOME_ASSISTANT`: Home Assistant endpoint called when the door is opened.
- `ENTITY_HOME_ASSISTANT`: Home Assistant entity targeted by that request, such as `light.insegna_o`.

The database file and its tables are created automatically when the backend starts.

## Docker

If running from MacOS, install `brew install colima`. Then start it with `colima start`. When done, `colima stop`.

### Build the image
This commands cross-compiles. Not necessary if building already from a Linux machine.
```bash
docker buildx build --platform linux/amd64 -t portonet:latest --load .
```

### Export the image to a tar file

```bash
docker save -o portonet.tar portonet:latest
```

### Deploy from the tar file

1. Copy `portonet.tar` to the target machine 
2. Create a `portonet-data` folder, copy there an existing `portonet.sqlite` file if available
3. `docker load -i portonet.tar`
4. `docker run -d --name portonet -p 8181:3000 -v /home/olli/portonet-data:/data portonet:latest`

Change the 8181 value to the desired port. 
