const API_LIST_KEYS = 'list_keys'
const API_ADD = 'add_key'
const API_COUNTERS = 'get_counters'
const API_REVOKE = 'revoke_key'
const API_REVOKE_ALL = 'revoke_all_keys'
const API_OPEN = 'open_door'
const API_LIST_LOGS = 'list_logs'

// Common variabile
let mode, interval, justTest

// Once DOM is loaded, attach events
document.addEventListener("DOMContentLoaded", () => {
    const submitBtn = document.getElementById('submit');
    const MPIn = document.getElementById('MP');
    const userkeyIn = document.getElementById('userkey');
    const loaderDiv = document.getElementById('loader');

    // Elements to show/hide
    const oreDiv = document.getElementById('oreGroup');
    const adminStuff = document.getElementById('adminpassword');
    const userStuff = document.getElementById('userpassword');

    // Modal
    const openMod = new bootstrap.Modal(document.getElementById('open'));
    const bodyMod = document.getElementById('bodyMod');

    // List
    const listOl = document.getElementById('list');

    // Manage mode button group
    document.getElementById('btnNuovo').addEventListener('click', () => {
        oreDiv.classList.remove('d-none');
        userStuff.classList.remove('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.add('d-none');
        mode = API_ADD;
    });
    document.getElementById('btnSvuota').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.add('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.add('d-none');
        mode = API_REVOKE_ALL;
    });
    document.getElementById('btnRevoca').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.remove('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.add('d-none');
        mode = API_REVOKE;
    });
    document.getElementById('btnTest').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.remove('d-none');
        adminStuff.classList.add('d-none');
        listOl.classList.add('d-none');
        mode = API_OPEN;
        justTest = 1; // TODO this is not implemented
    });
    document.getElementById('btnKeys').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.add('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.remove('d-none');
        mode = API_LIST_KEYS;
    });
    document.getElementById('btnLog').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.add('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.remove('d-none');
        mode = API_LIST_LOGS;
    });

    // Manage interval button group
    document.getElementById('btn6Ore').addEventListener('click', () => {
        interval = '6h';
    });
    document.getElementById('btn3Giorni').addEventListener('click', () => {
        interval = '3d';
    });
    document.getElementById('btn30Giorni').addEventListener('click', () => {
        interval = '30d';
    });
    document.getElementById('btn1Anno').addEventListener('click', () => {
        interval = '1y';
    });

    // Simulate click event when pressing enter
    MPIn.addEventListener('keydown', event => {
        if (event.key === 'Enter') {
            submitBtn.dispatchEvent(new Event('click'));
        }
    });
    userkeyIn.addEventListener('keydown', event => {
        if (event.key === 'Enter') {
            submitBtn.dispatchEvent(new Event('click'));
        }
    });

    // Main button submission
    submitBtn.addEventListener('click', () => {
        // Request options
        const options = {
            method: 'GET',
            timeout: 3000
        };
        // Build request url depending on action
        let url
        switch (mode) {
            case API_ADD:
                options.method = 'POST'
                url = `add_key?master_password=${MP.value}&new_key=${userkeyIn.value}&duration=${interval}`
                break
            case API_COUNTERS:
                url = 'get_counters'
                break
            case API_LIST_KEYS:
                url = `list_keys?master_password=${MP.value}`
                break
            case API_LIST_LOGS:
                url = `list_logs?master_password=${MP.value}&limit=50`
                break
            case API_OPEN:
                options.method = 'PUT'
                url = `open_door?u_key=${userkeyIn.value}`
                break
            case API_REVOKE:
                options.method = 'DELETE'
                url = `revoke_key?master_password=${MP.value}&key_to_revoke=${userkeyIn.value}`
                break
            case API_REVOKE_ALL:
                options.method = 'DELETE'
                url = `revoke_key?master_password=${MP.value}`
                break
            default:
                alert('Invalid mode: ' + mode)
                break;
        }

        fetch(url, options).then(response => {
            loaderDiv.style.display = "none";

            // Do not open the modal if we need to display some return data
            if (mode == API_LIST_KEYS) {
                response.json().then((j) => {
                    listOl.replaceChildren();
                    for (let i = 0; i < j.length; i++) {
                        let badgeClass = 'primary'
                        let badgeText = 'OK'
                        if (j[i].revoked == '1') {
                            badgeClass = 'danger'
                            badgeText = 'REVOKED'
                        } else if (Date.parse(j[i].exp_date) - Date.now() < 0) {
                            badgeClass = 'warning'
                            badgeText = 'EXPIRED'
                        }
                        let html = `<li class="list-group-item d-flex justify-content-between align-items-start">` +
                            `<div class="ms-2 me-auto">` +
                            `<div class="fw-bold fs-4">${j[i].ukey}</div>` +
                            `<b>Last used:</b> ${j[i].last_used}</br>` +
                            `<b>Expiration date:</b> ${j[i].exp_date}</br>` +
                            `<b>Used</b> ${j[i].n_used} times` +
                            `</div>` +
                            `<span class="badge bg-${badgeClass} rounded-pill">${badgeText}` +
                            `</span>` +
                            `</li>`;
                        listOl.innerHTML += html;
                    }
                });
            } else if (mode == API_LIST_LOGS) {
                response.json().then((j) => {
                    listOl.replaceChildren();
                    for (let i = 0; i < j.length; i++) {
                        let html = `<li class="list-group-item d-flex justify-content-between align-items-start">` +
                            `<div class="ms-2 me-auto">` +
                            `<div class="fw-bold fs-4">${j[i].api_name}</div>` +
                            `<b>Date:</b> ${j[i].request_date}</br>` +
                            `<b>Parameters:</b> ${j[i].params}</br>` +
                            `</div>` +
                            `</li>`;
                        listOl.innerHTML += html;
                    }
                });
            } else {
                listOl.classList.add('d-none');
                response.text().then((text) => {
                    bodyMod.innerHTML = `<b>HTTP Status Code: </b>${response.status}</br>`
                    if (text.length > 0)
                        bodyMod.innerHTML += `<b>Response: </b>${text}`;
                    openMod.show();
                });
            }


        });

        loaderDiv.style.display = "block";
    });


    /*
     * Init default state emulating clicks
     */
    document.getElementById('btnNuovo').click()
    document.getElementById('btn6Ore').click()
})

