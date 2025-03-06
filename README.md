# Libre Static Forms

Build native forms in static websites.

This is a dead simple middleware that simply handles POST requests from static websites and pipes them to a mailbox.

**Features:**
- Selfhostable, no external service used
- Multisite
- Free and Open Source accessible captcha support ([ALTCHA](https://altcha.org/))
- Generates HTML form from a JSON specification
- Uses HTML validation then checks received inputs server-side
- Sends an email to the form owner containing the form data
- Handles the following inputs : text, textarea, email and select. More inputs support is easy to implement.
- i18n mail templates

## hosts.json specification

The `hosts.json` file is used to define the fields of the forms you want to receive. Find its specification below.

```json
{
    // The website you want to receive forms from.
    // It actually doesn’t necessarily need to be a hostname, but it will identify the receiving endpoint
    // for the form, as you can handle multiple forms (thus multiple hosts) with this software.
    "hostname": {

        // Can be `fr` or `en`, but `en` is not fully supported yet
        "language": "fr",

        // the email address that will receive the form content.
        "recipient": "my-form-recipient@my-org.example.org",

        // Boolean to enable ALTCHA support in the form.
        "enable_captcha": true,

        // This section defines the received form inputs.
        "inputs": [
            {
                // the name used in 
                "display_name": "First name",

                // the field identifier
                "name": "first_name",

                // the input type. Can be text, textarea, email or select.
                "kind": "text",

                // indicates whether the field is required or not. Empty fields will be rejected.
                "required": true,

                // defines input type-specific parameters
                "settings": {
                    // defines the maximum input size. If not defined, will be 10000.
                    // does not apply to select inputs.
                    "maxlength": 100,

                    // Only for select inputs: defines all options.
                    "options": [
                        "Option A",
                        "Option B",
                        ...
                    ],
                    ...
                }
            },
            ...
        ]
    },
    ...
}
```

## How to use

1. Compile the binary or use the latest [Docker image](https://git.lacontrevoie.fr/lacontrevoie/-/packages/container/lsforms) here.

2. Create and fill the `config.toml` file from `config.toml.sample`.

3. Create and fill the `hosts.json` file from `hosts.json.sample`.
    - The sample file contains examples of various uses cases with the available features and supported inputs.
    - You will have to define all the inputs you want to handle.
    - You can add multiple hosts.

4. (Recommended) Compile the binary with the `templates` feature to generate the corresponding HTML form and integrate it into the target website.
    - `curl http://localhost:8888/{host}/gen_tpl`
    - You may add attributes or texts to your liking.

5. Host the binary on a server, behind the subdomain of your choice (let’s say: `static-forms.mydomain.org`). On the reverse proxy configuration, add the header `Access-Control-Allow-Origin *`.

6. Embed the following scripts into the web page that will host the form:

```html
<script async defer src="https://static-forms.mydomain.org/altcha.min.js" type="module"></script>
<script async defer src="https://static-forms.mydomain.org/form-response.js"></script>
```

- `altcha.min.js` : will load the ALTCHA widget if you integrate a captcha into your form.
- `form-response.js` : will handle form submission and display error/success messages.

7. Test your form, and you should be done.

## Binary features

The Cargo project has two available features:
- `static-files`: serves the `assets/` folder at the web root (`{host}/` routes gets priority), used to serve scripts and/or CSS.
    - Enabled by default.
- `templates`: generates the HTML form based on JSON definition in `hosts.json` using the route `/{host}/gen_tpl` route
    - Disabled by default ; if the feature is enabled, this route should be restricted/blocked in a production environment.
