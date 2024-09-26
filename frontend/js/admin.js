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

    // Remember the last button clicked
    let mode = 'add.php';
    let interval = 0;
    let justTest = 0;

    // Manage mode button group
    document.getElementById('btnNuovo').addEventListener('click', () => {
        oreDiv.classList.remove('d-none');
        userStuff.classList.remove('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.add('d-none');
        mode = 'add.php';
    });
    document.getElementById('btnSvuota').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.add('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.add('d-none');
        mode = 'revokeAll.php';
    });
    document.getElementById('btnRevoca').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.remove('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.add('d-none');
        mode = 'revoke.php';
    });
    document.getElementById('btnTest').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.remove('d-none');
        adminStuff.classList.add('d-none');
        listOl.classList.add('d-none');
        mode = 'enter.php';
        justTest = 1;
    });
    document.getElementById('btnKeys').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.add('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.remove('d-none');
        mode = 'keyList.php';
    });
    document.getElementById('btnLog').addEventListener('click', () => {
        oreDiv.classList.add('d-none');
        userStuff.classList.add('d-none');
        adminStuff.classList.remove('d-none');
        listOl.classList.remove('d-none');
        mode = 'logList.php';
    });

    // Manage interval button group
    document.getElementById('btn6Ore').addEventListener('click', () => {
        interval = 0;
    });
    document.getElementById('btn3Giorni').addEventListener('click', () => {
        interval = 1;
    });
    document.getElementById('btn30Giorni').addEventListener('click', () => {
        interval = 2;
    });
    document.getElementById('btn1Anno').addEventListener('click', () => {
        interval = 3;
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
        const url = `API/${mode}?MP=${MP.value}&uKey=${userkeyIn.value}&interval=${interval}&justTest=${justTest}`;
        const options = {
            method: "GET",
            timeout: 3000
        };

        fetch(url, options).then(response => {
            loaderDiv.style.display = "none";

            // Do not open the modal if we need to display the list
            if (mode == 'keyList.php') {
                response.json().then((j) => {
                    listOl.replaceChildren();
                    for (let i = 0; i < j.length; i++) {
                        let badgeClass = 'primary'
                        let badgeText = 'OK'
                        if (j[i].revoked == '1') {
                            badgeClass = 'danger'
                            badgeText = 'REVOKED'
                        } else if (Date.parse(j[i].expDate) - Date.now() < 0) {
                            badgeClass = 'warning'
                            badgeText = 'EXPIRED'
                        }
                        let html = `<li class="list-group-item d-flex justify-content-between align-items-start">` +
                            `<div class="ms-2 me-auto">` +
                            `<div class="fw-bold fs-4">${j[i].uKey}</div>` +
                            `<b>Last used:</b> ${j[i].lastUsed}</br>` +
                            `<b>Expiration date:</b> ${j[i].expDate}</br>` +
                            `<b>Used</b> ${j[i].nUsed} times` +
                            `</div>` +
                            `<span class="badge bg-${badgeClass} rounded-pill">${badgeText}` +
                            `</span>` +
                            `</li>`;
                        listOl.innerHTML += html;
                    }
                });
            } else if (mode == 'logList.php') {
                response.json().then((j) => {
                    listOl.replaceChildren();
                    for (let i = 0; i < j.length; i++) {
                        let html = `<li class="list-group-item d-flex justify-content-between align-items-start">` +
                            `<div class="ms-2 me-auto">` +
                            `<div class="fw-bold fs-4">${j[i].APIName}</div>` +
                            `<b>Date:</b> ${j[i].dateRequest}</br>` +
                            `<b>Parameters:</b> ${j[i].params}</br>` +
                            `</div>` +
                            `</li>`;
                        listOl.innerHTML += html;
                    }
                });
            } else {
                listOl.classList.add('d-none');
                response.text().then((text) => {
                    bodyMod.innerHTML = `<b>HTTP Status Code: </b>${response.status}</br>
                    <b>Response: </b>${text}`;
                    openMod.show();
                });
            }


        });

        loaderDiv.style.display = "block";
    });
})

