let star_view_only = false;
let star_edit_mode = false;

let api_global_stars = [];
let api_own_stars = {};
let api_own_stars_result = {};
let data_stars = [];
let startoken = null;

window.onload = async function() {
    // First load stars
    await fetchData("/data/stars.json", function(d) { data_stars = d; });
    await fetchData("/api/stars/global", function(d) { api_global_stars = d; });
    await loadStars();

    // check if the user is about to place their stars
    let params = new URLSearchParams(document.location.search);
    startoken = params.get("token");
    if (startoken != null) {
        // get gem data
        let get_url = `/api/stars/own?token=${startoken}`;
        await fetchData(get_url, function(d) { api_own_stars = d; });
        if (api_own_stars.code != undefined && api_own_stars.code == 3001) {
            window.location = "/";
        }
        get("star-selector").classList.remove("hidden");
    } else {
        star_view_only = true;
    }

    let star_send = get("star-send-button");

    if (!star_view_only) {
        await loadStarSelector();
        await updateGemCounter();

        get("star-send-button").innerHTML = "Envoyer vos étoiles ✨"
        get("star-send-button").removeAttribute("href");
        get("star-send-button").addEventListener('click', sendStars);
        get("star-confirm-cancel").addEventListener('click', cancelStarSend);
        get("star-confirm-send").addEventListener('click', confirmStarSend);
        get("send-box-username").value = api_own_stars.username;
        get("send-box-message").value = api_own_stars.message;
        get("tutorial").classList.remove("hidden");
    }

}

function cancelStarSend() {
    get("send-box-modal").classList.add("hidden");
}

async function confirmStarSend() {
    let sendbutton = get("star-confirm-send");

    sendbutton.classList.add("in-progress");
    sendbutton.innerHTML = "Envoi…";

    // defining OwnTokenPost webmodel
    let jsondata = {};
    jsondata["token"] = startoken;
    jsondata["username"] = escapeHTML(get("send-box-username").value);
    jsondata["message"] = escapeHTML(get("send-box-message").value);
    jsondata["stars"] = [];

    // collect all stars
    let star_list_elem = document.getElementsByClassName("own-canvas");

    for (let i = 0; i < star_list_elem.length; i++) {
        let newstarpost = {};
        newstarpost["startype"] = parseInt(star_list_elem[i].dataset.startype);
        newstarpost["position_x"] = parseFloat(star_list_elem[i].style.left.replace('%', ""));
        newstarpost["position_y"] = parseFloat(star_list_elem[i].style.top.replace('%', ""));
        jsondata["stars"].push(newstarpost);
    }

    await fetchData(
        "/api/stars/own",
        function(d) { api_own_stars_result = d; },
        "POST",
        jsondata,
    );

    if (api_own_stars_result.code != undefined && api_own_stars_result.code == 1001) {
        window.location = "/";
    }

}

async function sendStars() {
    get("send-box-modal").classList.remove("hidden");
}


async function loadStars() {
    for (let i = 0; i < api_global_stars.length; i++) {
        putStar(api_global_stars[i], false);
    }
}

async function loadStarSelector() {
    // add and display star selector menu
    get("star-selector-toggler").addEventListener("click", e_toggle_star_select);
    for (let i = 0; i < data_stars.length; i++) {
        let star_item = document.createElement("div");

        let star_item_img = document.createElement("img");
        star_item_img.src = data_stars[i].path;
        star_item_img.style.width = `${data_stars[i].size_pct}%`
        star_item_img.style.height = `${data_stars[i].size_pct}%`
        star_item.appendChild(star_item_img);

        let star_item_price = document.createElement("div");
        star_item_price.classList.add("star-price");
        star_item_price.innerHTML = data_stars[i].price;
        star_item.appendChild(star_item_price);

        star_item.classList.add("star-item");
        star_item.dataset.startype = data_stars[i].startype;
        star_item.dataset.path = data_stars[i].path;
        star_item.dataset.price = data_stars[i].price;
        star_item.dataset.size_pct = data_stars[i].size_pct;

        star_item.addEventListener("click", e_hold_star);
        get("tab-1-content").appendChild(star_item);
    }

    let own_canvas = get("star-canvas");
    // listen on star placements in edit mode
    own_canvas.addEventListener("click", e_drop_star);
    own_canvas.addEventListener("mouseenter", e_mouseenter_star_edit);
    own_canvas.addEventListener("mouseleave", e_mouseleave_star_edit);
}

function putStar(stardata, isOwn) {
    let new_star = document.createElement("img");
    let canvas = get("star-canvas");

    if (isOwn) {
        new_star.classList.add("own-canvas");
        // add function to remove ownstar by clicking on it
        new_star.addEventListener("click", () => { e_click_ownstar(new_star)});
    } else {
        new_star.classList.add("global-canvas");
        // add globalstar tooltip
        new_star.addEventListener("mouseenter", () => { e_mouseenter_globalstar(stardata)});
        new_star.addEventListener("mouseleave", () => { e_mouseleave_globalstar(stardata)});
    }

    let starinfo = data_stars.find(star => star.startype == stardata.startype);
    new_star.dataset.price = starinfo.price;
    new_star.dataset.startype = starinfo.startype;
    new_star.src = starinfo.path;
    new_star.style.left = `${stardata.position_x}%`;
    new_star.style.top = `${stardata.position_y}%`;
    new_star.style.width = `calc(6% * ${starinfo.size_pct / 100})`
    new_star.style.height = `calc(6% * ${starinfo.size_pct / 100})`
    canvas.appendChild(new_star);

    if (isOwn) {
        updateGemCounter();
    }
}

function e_click_ownstar(new_star) {
    if (star_edit_mode === true) {
        return ;
    }
    new_star.remove();
    updateGemCounter();
}

function e_mouseenter_globalstar(stardata) {
    // do not display tooltip in edit mode
    if (star_edit_mode === true) {
        return ;
    }
    let tooltip = get("mouse-tooltip-globalstar");

    let frac = stardata.position_x / 100;
    if (frac > 0.7) {
        tooltip.style.left = `calc(${stardata.position_x}% - ${tooltip.clientWidth}px - 20px)`;
    }
    else {
        tooltip.style.left = `calc(${stardata.position_x}% + 20px)`;
    }
    tooltip.style.top = `calc(${stardata.position_y}% + 20px)`;
    let day = new Date(stardata.day);
    let day_fmt = day.toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' });
    tooltip.innerHTML = `
    <div class="star-username">${(stardata.username.length == 0 ? "Anonyme" : stardata.username)}</div>
    <div class="star-date">le ${day_fmt}</div>
    ${stardata.message.length == 0 ? "" : `<div class="star-message">«&nbsp;${stardata.message}&nbsp;»</div>`}`;
    if (tooltip.classList.contains("opacity-hidden")) {
        tooltip.classList.remove("opacity-hidden");
    }
}

function e_mouseleave_globalstar(stardata) {
    let tooltip = get("mouse-tooltip-globalstar");
    if (!tooltip.classList.contains("opacity-hidden")) {
        tooltip.classList.add("opacity-hidden");
    }
}

function e_toggle_star_select() {
    let star_selector = get("star-selector");
    if (star_selector.classList.contains("collapsed")) {
        star_selector.classList.remove("collapsed");
        get("star-selector-toggler").innerHTML = "▼";
    } else {
        star_selector.classList.add("collapsed");
        get("star-selector-toggler").innerHTML = "▲";
    }
}

function e_mouseenter_star_edit() {
    if (star_edit_mode) {
        let tooltip_star = get("mouse-tooltip-ownstar");
        if (tooltip_star.classList.contains("hidden")) {
            tooltip_star.classList.remove("hidden");
        }
    }
}

function e_mouseleave_star_edit() {
    if (star_edit_mode) {
        let tooltip_star = get("mouse-tooltip-ownstar");
        if (!tooltip_star.classList.contains("hidden")) {
            tooltip_star.classList.add("hidden");
        }
    }
}

function disable_star_edit() {
    let tooltip_star = get("mouse-tooltip-ownstar");
    let own_canvas = get("star-canvas");

    own_canvas.classList.remove("edit-mode");
    tooltip_star.classList.add("hidden");
    tooltip_star.removeAttribute("data-name");
    tooltip_star.removeAttribute("data-startype");
    tooltip_star.removeAttribute("data-price");
    own_canvas.removeEventListener('mousemove', e_follow_star);
    tooltip_star.innerHTML = ``;
    star_edit_mode = false;
}

function clear_selected_stars() {
    let stars_list = document.getElementsByClassName("star-item");
    for (let i = 0; i < stars_list.length; i++) {
        if (stars_list[i].classList.contains("selected")) {
            stars_list[i].classList.remove("selected");
        }
    }
    let tooltip_star = get("mouse-tooltip-ownstar");
    tooltip_star.innerHTML = ``;
}

function e_hold_star() {
    let tooltip_star = get("mouse-tooltip-ownstar");
    let own_canvas = get("star-canvas");
    // if the user clicks on the same star: disable edit mode
    if (tooltip_star.dataset.startype == this.dataset.startype) {
        this.classList.remove("selected");
        disable_star_edit();
        return ;
    }

    if (tooltip_star.dataset.startype == null) {
        own_canvas.classList.add("edit-mode");
    } else {
        clear_selected_stars();
    }
    star_edit_mode = true;
    this.classList.add("selected");

    // if another star is clicked while in edit mode: replace star
    // prepare tooltip with star
    tooltip_star.dataset.startype = this.dataset.startype;
    tooltip_star.dataset.price = this.dataset.price;
    tooltip_star.dataset.size_pct = this.dataset.size_pct;
    let tooltip_img = document.createElement("img");
    tooltip_img.src = `${this.dataset.path}`;
    tooltip_star.appendChild(tooltip_img);
    tooltip_star.style.width = `calc(6% * ${this.dataset.size_pct / 100})`
    tooltip_star.style.height = `calc(6% * ${this.dataset.size_pct / 100})`
    
    // listen on star placements in edit mode
    own_canvas.addEventListener('mousemove', function(e) { e_follow_star(e, tooltip_star) });
}

// display star tooltip during edit mode
function e_follow_star(e, tooltip) {
    if (star_edit_mode === false) {
        return ;
    }
    let frac = e.pageX / document.body.clientWidth;
    if (frac > 0.7) {
        tooltip.style.left = (e.pageX - 10 - tooltip.clientWidth) + 'px';
    }
    else {
        tooltip.style.left = (e.pageX + 10) + 'px';
    }
    tooltip.style.top = (e.pageY + 10) + 'px';

    // if dropping the star would cause a canvas overflow,
    // prevent the user from dropping the star here
    let canvas = get("star-canvas");
    if (((e.pageX + tooltip.clientWidth + 2) > canvas.clientWidth)
    || ((e.pageY + tooltip.clientHeight + 2) > canvas.clientHeight)) {
        if (!canvas.classList.contains("outscope")) {
            canvas.classList.add("outscope");
        }
    } else {
        if (canvas.classList.contains("outscope")) {
            canvas.classList.remove("outscope");
        }
    }
}

// drop star if holding one.
function e_drop_star(e) {
    if (star_edit_mode === false || get("star-canvas").classList.contains("outscope")) {
        return ;
    }
    let star_tooltip = get("mouse-tooltip-ownstar");
    let star_json = {};
    let own_canvas = get("star-canvas");


    star_json["startype"] = star_tooltip.dataset.startype;
    star_json["path"] = star_tooltip.dataset.path;
    star_json["position_x"] = ((e.pageX - star_tooltip.clientWidth / 2) / own_canvas.clientWidth) * 100;
    star_json["position_y"] = ((e.pageY - star_tooltip.clientHeight / 2) / own_canvas.clientHeight) * 100;
    putStar(star_json, true);

    disable_star_edit();
    clear_selected_stars();
}

async function updateGemCounter() {
    let gems_left = api_own_stars.gems - countGems();
    get("gem-counter").innerHTML = `Il vous reste <b>${gems_left} fragments d’étoile</b> !`;

    let available_stars = get("tab-1-content");

    for (let i = 0; i < available_stars.children.length; i++) {
        if (gems_left < available_stars.children[i].dataset.price) {
            if (!available_stars.children[i].classList.contains("disabled")) {
                available_stars.children[i].classList.add("disabled");
            }
        } else {
            if (available_stars.children[i].classList.contains("disabled")) {
                available_stars.children[i].classList.remove("disabled");
            }
        }
    }
    if (api_own_stars.gems == gems_left) {
        if (!get("star-send-button").classList.contains("disabled")) {
            get("star-send-button").classList.add("disabled");
        }
    } else {
        if (get("star-send-button").classList.contains("disabled")) {
            get("star-send-button").classList.remove("disabled");
        }
    }
}

function countGems() {
    let canvas = get("star-canvas");

    let totalPrice = 0;

    for (let i = 0; i < canvas.children.length; i++) {
        if (canvas.children[i].classList.contains("own-canvas")) {
            totalPrice += parseInt(canvas.children[i].dataset.price);
        }
    }

    return totalPrice;
}
