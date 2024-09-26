// Update number of correct and failed logons
function getCounters() {
    const url = "get_counters";
    const options = {
        method: "GET",
        timeout: 3000
    }
    fetch(url, options).then(response => {
        if (response.ok) {
            const count1P = document.getElementById('count1');
            const count2P = document.getElementById('count2');

            response.text().then(text => {
                q = JSON.parse(text);
                count1P.innerText = `Totale aperture: ${q.n_openings}`;
                count2P.innerText = `Tentativi falliti: ${q.n_errors}`;
            })
        } else {
            count2P.innerText = count1P.innerText = "-E-";
        }
    });
}

// Once DOM is loaded, attach events
document.addEventListener("DOMContentLoaded", () => {

    const keyIn = document.getElementById('key');
    const accessForm = document.getElementById('access');
    const submitBtn = document.getElementById('submit');
    const loaderDiv = document.getElementById('loader');

    // Modal
    const openMod = new bootstrap.Modal(document.getElementById('open'));
    const bodyMod = document.getElementById('bodyMod');

    // Simulate click event when pressing enter
    keyIn.addEventListener('keydown', event => {
        if (event.key === 'Enter') {
            submitBtn.dispatchEvent(new Event('click'));
        }
    });

    // Main button submission
    submitBtn.addEventListener('click', () => {
        const url = `API/enter.php?uKey=${keyIn.value}`;
        const options = {
            method: "GET",
            timeout: 3000
        };

        fetch(url, options).then(response => {
            loaderDiv.style.display = "none";
            response.text().then((text) => {
                bodyMod.innerHTML = `<b>HTTP Status Code: </b>${response.status}</br>
                    <b>Response: </b>${text}`;
            });

            openMod.show();
            getCounters();
        });

        loaderDiv.style.display = "block";
    });

    getCounters();
});