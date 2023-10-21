let api_transactions = [];
let email_templates = [];

window.onload = async function() {
    await fetchData("/admin/api/transaction", function(d) { api_transactions = d; });
    loadTransactions();
    await fetchData("/admin/api/email_templates", function(d) { email_templates = d; });
    loadEmailTemplates();
}

function loadTransactions() {
    // id, username, message, email, day, amount, gems, ha_id,
    // token, receipt, event_type, is_mail_sent, is_token_used, is_checked
    get("trans-table-body").innerHTML = `
        ${api_transactions.map((tr, index) => `
            <tr id="tr-${tr.id}-row" class="tr-row ${(tr.is_checked ? "is_checked" : "")}">
                <td>${tr.id}</td>
                <td>${tr.username}</td>
                <td><span title="${tr.message}">${(tr.message == "" ? "" : "(voir)")}</span></td>
                <td>${tr.email}</td>
                <td>${tr.day}</td>
                <td>${tr.amount}</td>
                <td>${tr.gems}</td>
                <td>${tr.ha_id}</td>
                <td><span title="${tr.token}">(voir)</span></td>
                <td><a href="${tr.receipt_url}">PDF</a></td>
                <td data-type="${tr.event_type}">${tr_event_type(tr.event_type)}</td>
                <td>${tr_bool(tr.is_mail_sent)} <button id="tr-${tr.id}-button" onclick="resendMail(${tr.id})">${(tr.is_mail_sent ? "Renvoyer…" : "Envoyer…" )}</button></td>
                <td>${tr_bool(tr.is_token_used)}</td>
                <td><input type="checkbox" ${(tr.is_checked ? "checked" : "")} onclick="checkTransaction(${tr.id})" /></td>
            </tr>
        `).join("")}`;
}

function loadEmailTemplates() {
    get("templates-list").innerHTML = `
        <div>
            <label>Marquer commme envoyé</label>
            <button id="b-0" class="big" onclick="resendMailConfirm(0)">Envoyer</button>
        </div>
        ${email_templates.map((e, index) => `
        <div>
            <label>${e.name}</label>
            <button id="b-${e.id}" class="big" onclick="resendMailConfirm(${e.id})">Envoyer</button>
        </div>
        `).join("")}
        <button class="big center" onclick="resendMailCancel()">Annuler</button>
    `;
}

function resendMailCancel() {
    get("send-box-modal").classList.add("hidden");

    let templates = get("templates-list");
    for (let i = 0; i < templates.children.length; i++) {
        if (templates.children[i].classList.contains("advice-maybe")) {
            templates.children[i].classList.remove("advice-maybe");
        }
        if (templates.children[i].classList.contains("advice-highlight")) {
            templates.children[i].classList.remove("advice-highlight");
        }
    }
}

function tr_event_type(t) {
    switch (t) {
        case 0: return "Adhésion (commande)";
        case 1: return "Don (commande)";
        case 2: return "Inconnu (commande)";
        case 3: return "Don mensuel (paiement)";
        case 4: return "Adhésion (paiement)";
        case 5: return "Don (paiement)";
        case 6: return "Inconnu (paiement)";
        default: return "Inconnu";
    }
}

function tr_bool(b) {
    return (b == true ? "Oui" : "Non");
}

async function resendMailConfirm(id) {
    // id = mail template id
    // tr_id = transaction id
    let btn = get(`b-${id}`);
    btn.disabled = true;
    btn.innerHTML = "Envoi en cours…";
    btn.classList.add("is-sending");
    let tr_id = parseInt(get("title-username").dataset.id);
    await fetchData(`/admin/api/transaction/${tr_id}/send_mail/${id}`, function(d) {
        if (d.code != undefined && d.code == 1002) {
            alert("Mail envoyé !");
            window.location.reload(true);
        }
    }, "POST");
}

function resendMail(tr_id) {
    // get username field
    let username = get(`tr-${tr_id}-row`).children[1].innerHTML;
    if (username == "") {
        // get email field as fallback
        username = get(`tr-${tr_id}-row`).children[3].innerHTML;
    }
    get("title-username").innerHTML = username;
    get("title-username").dataset.id = tr_id;
    printAdvice(tr_id);
    get("send-box-modal").classList.remove("hidden");
}

function printAdvice(tr_id) {
    let row = get(`tr-${tr_id}-row`);
    let event_type = parseInt(row.children[10].dataset.type);
    let templates = get("templates-list");

    let advice = "";

    switch (event_type) {
        case 0:
            advice += "Il s’agit d’une <b>adhésion</b> simple.";
            break;
        case 1:
            advice += "Il s’agit d’un <b>don</b> simple.";
            break;
        case 2:
            advice += "Il s’agit d’une <b>commande non reconnue par Constello</b>. Son apparition peut être liée à l’utilisation de fonctionnalités inhabituelles d’HelloAsso ou un changement de leur API.";
            break;
        case 3:
            advice += "Il s’agit d’un <b>don mensuel</b>, il convient donc de ne pas envoyer de mail, sauf pour le premier don.";
            templates.children[0].classList.add("advice-highlight");
            templates.children[1].classList.add("advice-maybe");
            templates.children[2].classList.add("advice-maybe");
            templates.children[5].classList.add("advice-maybe");
            templates.children[6].classList.add("advice-maybe");
            break;
        case 4:
            advice += "Il s’agit d’un <b>paiement d’adhésion</b> sans sa commande d’adhésion. C’est étrange que cette entrée existe… Elle peut être liée à une manière inhabituelle pour un·e membre de régler une adhésion. Quoi qu’il en soit, c’est une adhésion.";
            break;
        case 5:
            advice += "Il s’agit d’un <b>paiement de don</b>. Cette action particulière existe généralement lorsqu’une adhésion est couplée avec un don. Si un mail de remerciement d’adhésion a déjà été envoyé, peut-être qu’envoyer un deuxième mail pour le don n’en vaut pas la peine, sauf s’il s’agit d’un gros montant.";
            templates.children[0].classList.add("advice-highlight");
            break;
        case 6:
            advice += "C’est un <b>paiement non reconnu par Constello</b>. Son existence est sans doute liée à l’utilisation de fonctionnalités inhabituelles d’HelloAsso ou un changement dans leur API.";
            break;
        default:
            advice += "Il y a un bug quelque part. Ou quelqu’un a saisi un mauvais event_type. Tu tu tu.";
            break;
    }

    if (event_type == 0 || event_type == 4) {
        advice += "<br />Il faut déterminer à l’aide de la comptabilité s’il s’agit d’une <b>première adhésion</b> ou d’un <b>renouvellement</b> et envoyer le mail correspondant.";
        templates.children[3].classList.add("advice-maybe");
        templates.children[4].classList.add("advice-maybe");
        templates.children[7].classList.add("advice-highlight");
        templates.children[8].classList.add("advice-highlight");
    }
    if (event_type == 1 || event_type == 5) {
        advice += "<br />Si une campagne de dons est en cours, il faut envoyer la variante « Don en campagne » qui communique le lien pour poser une étoile.";
        templates.children[1].classList.add("advice-maybe");
        templates.children[2].classList.add("advice-maybe");
        templates.children[5].classList.add("advice-highlight");
        templates.children[6].classList.add("advice-highlight");
    }
    advice += "<br />Par défaut, le vouvoiement est de rigueur sauf si l’on connaît la personne.";

    get("advice").innerHTML = advice;
}

async function checkTransaction(tr_id) {
    await fetchData(`/admin/api/transaction/${tr_id}/toggle_check`, function(d) {
        if (d.code != undefined && d.code == 1003) {
            get(`tr-${tr_id}-row`).classList.toggle("is_checked");
        }
    }, "POST");
}
