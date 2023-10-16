let api_transactions = [];

window.onload = async function() {
    await fetchData("/admin/api/transaction", function(d) { api_transactions = d; });

    loadTransactions();
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
                <td>${tr_event_type(tr.event_type)}</td>
                <td>${tr_bool(tr.is_mail_sent)} <button id="tr-${tr.id}-button" onclick="resendMail(${tr.id})">${(tr.is_mail_sent ? "Renvoyer" : "Envoyer" )}</button></td>
                <td>${tr_bool(tr.is_token_used)}</td>
                <td><input type="checkbox" ${(tr.is_checked ? "checked" : "")} onclick="checkTransaction(${tr.id})" /></td>
            </tr>
        `).join("")}`;
}

function tr_event_type(t) {
    switch (t) {
        case 0: return "Adhésion (commande)";
        case 1: return "Adhésion+don (commande)";
        case 2: return "Don (commande)";
        case 3: return "Inconnu (commande)";
        case 4: return "Don mensuel (paiement)";
        case 5: return "Adhésion (paiement)";
        case 6: return "Don (paiement)";
        case 7: return "Inconnu (paiement)";
        default: return "Inconnu";

    }
}

function tr_bool(b) {
    return (b == true ? "Oui" : "Non");
}

async function resendMail(tr_id) {
    get(`tr-${tr_id}-button`).disabled = true;
    await fetchData(`/admin/api/transaction/${tr_id}/send_mail`, function(d) {
        if (d.code != undefined && d.code == 1002) {
            alert("Mail envoyé !");
        }
    }, "POST");
}

async function checkTransaction(tr_id) {
    await fetchData(`/admin/api/transaction/${tr_id}/toggle_check`, function(d) {
        if (d.code != undefined && d.code == 1003) {
            get(`tr-${tr_id}-row`).classList.toggle("is_checked");
        }
    }, "POST");
}
