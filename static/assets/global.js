async function fetchData(filename, fn_results, method = "GET", body = null) {
    await fetch(filename, {
        method: method,
        body: body,
    })
        .then(function (response) {
            return response.json();
        })
        .then(fn_results)
        .catch(function (err) {
            console.log(err);
            alert("Le chargement des étoiles a échoué. Veuillez actualiser la page.");
        });
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}


// to use when displaying unsafe chars in html
function escapeHTML(unsafe) {
    if (unsafe != null) {
        return unsafe
            .replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;")
            .replace(/"/g, "&quot;")
                .replace(/'/g, "&#039;");
    }
    else {
        return null;
    }
}

function get(elem) {
    return document.getElementById(elem);
}
