const locale_fr = {
    "CaptchaResultInvalid": "Le captcha est invalide, veuillez réessayer.",
};

window.addEventListener("load", async () => {

    document.getElementById("lcv-form").addEventListener("submit", (event) => {

        event.preventDefault();
        event.target.reportValidity();
        if (event.target.checkValidity() === false) {
            return;
        }

        /* prevent the user from pressing the button multiple times */
        let submit_button = document.getElementById("lcv-form-submit");
        submit_button.value = "Envoi en cours...";
        submit_button.disabled = true;

        /* sending the form data */
        fetch(event.target.action, {
            method: 'POST',
            body: new URLSearchParams(new FormData(event.target))
        }).then((response) => {
            return response.json();
        }).then((body) => {
            if (body.status == "ok") {
                formSuccess(body);
            } else if (body.status == "error") {
                formError(body);
            }
        }).catch((error) => {
            formError("network");
        });
    });
});

function formError(body) {
    if (body === "network") {
        alert("L’envoi du formulaire a échoué, vérifiez votre connexion Internet. Si elle n’est pas en cause, contactez les gestionnaires du site web.");
    } else {
        if (locale_fr[body.error_kind] == undefined) {
            alert(`L’envoi du formulaire a échoué, code d’erreur ${body.code}. Veuillez contacter les gestionnaires du site web.`);
        } else {
            alert(`L’envoi du formulaire a échoué : ${locale_fr[body.error_kind]} Si cette erreur persiste, veuillez contacter les gestionnaires du site web.`);
        }
    }
    enableSubmitButton();
}

function formSuccess(body) {
    document.getElementById("lcv-form").outerHTML = "<div><p><b>Merci, le formulaire a été envoyé !</b></p></div>";
}

function enableSubmitButton() {
    let submit_button = document.getElementById("lcv-form-submit");
    submit_button.value = "Envoyer";
    submit_button.disabled = false;
}
