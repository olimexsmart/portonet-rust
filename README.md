Learn Rust while refreshing a very useful project of mine. 
### GOALS V1
- [ ] Rewrite all PHP functionality in Rust
- [ ] Use SQLite or PostgreSQL?
- [ ] Do not modify frontend for the moment except for some API calls
- [ ] Make it a system service that start at startup
- [ ] Configure Apache proxy to point the backend
### GOALS V2
- [ ] Bump frontend to a recent Bootstrap version
- [ ] Make it an Angular WebApp
- [ ] Implement Web AUTH for biometric login

### APIs
- `/add_key`
	- Inputs: `[masterPassword, newKey, duration]`
	- Verify master password validity and then insert new key with the specified duration
	- If the key is already present, duration should be updated instead (key duration refresh)
- `/get_counters
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
	        "revoked": boolen
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

### More Backend
- [ ] Every API call is logged along with the parameters and IP
- [ ] The door is opened through an API call configurable via ENV file
- [ ] If system table is empty should be initialized 
- [ ] Tables should be created if not existent
- [ ] Every API that checks master password should also be subject to system block to avoid brute forcing

### Run with auto-reload
```
cargo watch -c -w src/ -x 'run'
```