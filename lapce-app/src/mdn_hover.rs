use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::config::LapceConfig;
use crate::markdown::{MarkdownContent, parse_markdown};

pub enum MdnCategory {
    Html,
    Css,
    JavaScript,
}

pub struct MdnEntry {
    pub category: MdnCategory,
    pub title: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub example: Option<&'static str>,
}

impl MdnEntry {
    pub fn to_markdown(&self, config: &LapceConfig) -> Vec<MarkdownContent> {
        let mut text = format!(
            "#### `{}`\n\n{}\n\n[📖 MDN Reference]({})",
            self.title, self.description, self.url
        );
        if let Some(example) = self.example {
            text = format!("{}\n\n```\n{}```", text, example);
        }
        parse_markdown(&text, 1.8, config)
    }

    pub fn category_name(&self) -> &'static str {
        match self.category {
            MdnCategory::Html => "HTML",
            MdnCategory::Css => "CSS",
            MdnCategory::JavaScript => "JavaScript",
        }
    }
}

static MDN_REFERENCES: Lazy<HashMap<&'static str, MdnEntry>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // ── HTML ──────────────────────────────────────────────────────────────
    macro_rules! html {
        ($name:expr, $desc:expr, $url:expr, $ex:expr) => {
            m.insert($name, MdnEntry {
                category: MdnCategory::Html,
                title: concat!("&lt;", $name, "&gt;"),
                description: $desc,
                url: concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", $url),
                example: $ex,
            });
        };
    }
    html!("html", "The root element of an HTML document.", "html", Some("<html lang=\"en\">\n  <head>...</head>\n  <body>...</body>\n</html>"));
    html!("head", "Container for metadata, title, scripts, and styles.", "head", Some("<head>\n  <meta charset=\"UTF-8\">\n  <title>Page Title</title>\n</head>"));
    html!("body", "The main content area of the document.", "body", Some("<body>\n  <h1>Hello World</h1>\n</body>"));
    html!("div", "A generic flow container with no semantic meaning.", "div", Some("<div class=\"container\">\n  <p>Content</p>\n</div>"));
    html!("span", "An inline generic container for phrasing content.", "span", Some("<p>This is <span class=\"highlight\">highlighted</span> text.</p>"));
    html!("a", "Creates a hyperlink to another page or resource.", "a", Some("<a href=\"https://example.com\">Click here</a>"));
    html!("p", "Defines a paragraph of text.", "p", Some("<p>This is a paragraph of text.</p>"));
    html!("h1", "Top-level section heading.", "h1", Some("<h1>Main Page Heading</h1>"));
    html!("h2", "Second-level section heading.", "h2", None);
    html!("h3", "Third-level section heading.", "h3", None);
    html!("h4", "Fourth-level section heading.", "h4", None);
    html!("h5", "Fifth-level section heading.", "h5", None);
    html!("h6", "Sixth-level (lowest) section heading.", "h6", None);
    html!("img", "Embeds an image in the document.", "img", Some("<img src=\"photo.jpg\" alt=\"Description\" width=\"400\">"));
    html!("input", "A form input control of various types.", "input", Some("<input type=\"text\" placeholder=\"Enter name\">"));
    html!("button", "A clickable button element.", "button", Some("<button type=\"submit\">Submit</button>"));
    html!("form", "A section containing form controls for user input.", "form", Some("<form action=\"/submit\" method=\"POST\">\n  <input type=\"text\" name=\"name\">\n  <button>Send</button>\n</form>"));
    html!("ul", "An unordered (bulleted) list.", "ul", Some("<ul>\n  <li>Item one</li>\n  <li>Item two</li>\n</ul>"));
    html!("ol", "An ordered (numbered) list.", "ol", Some("<ol>\n  <li>First</li>\n  <li>Second</li>\n</ol>"));
    html!("li", "A single item in a list.", "li", None);
    html!("table", "A data table with rows and columns.", "table", Some("<table>\n  <tr>\n    <th>Name</th>\n    <th>Age</th>\n  </tr>\n  <tr>\n    <td>Alice</td>\n    <td>30</td>\n  </tr>\n</table>"));
    html!("tr", "A table row.", "tr", None);
    html!("td", "A table data cell.", "td", None);
    html!("th", "A table header cell.", "th", None);
    html!("header", "Introductory content or navigational aids.", "header", None);
    html!("footer", "Footer content for a section or page.", "footer", None);
    html!("nav", "A section with navigation links.", "nav", None);
    html!("section", "A standalone section of content.", "section", None);
    html!("article", "A self-contained composition (blog post, news story).", "article", None);
    html!("aside", "Content tangentially related to the main content.", "aside", None);
    html!("main", "The dominant content of the document.", "main", None);
    html!("meta", "Metadata such as charset, viewport, and description.", "meta", Some("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"));
    html!("link", "Links external resources (CSS, favicon, etc.).", "link", Some("<link rel=\"stylesheet\" href=\"styles.css\">"));
    html!("script", "Embeds or references executable JavaScript.", "script", Some("<script src=\"app.js\"></script>"));
    html!("style", "Contains CSS style rules for the document.", "style", Some("<style>\n  body { font-family: sans-serif; }\n</style>"));
    html!("title", "Sets the document title shown in the browser tab.", "title", Some("<title>My Web Page</title>"));
    html!("video", "Embeds a video player in the document.", "video", Some("<video src=\"movie.mp4\" controls></video>"));
    html!("audio", "Embeds an audio player.", "audio", Some("<audio src=\"song.mp3\" controls></audio>"));
    html!("canvas", "A drawable bitmap canvas for 2D/3D graphics.", "canvas", Some("<canvas id=\"myCanvas\" width=\"300\" height=\"200\"></canvas>"));
    html!("iframe", "Embeds another HTML document inline.", "iframe", Some("<iframe src=\"https://example.com\"></iframe>"));
    html!("label", "A label for a form control.", "label", Some("<label for=\"email\">Email:</label>\n<input id=\"email\" type=\"email\">"));
    html!("select", "A dropdown selection control.", "select", Some("<select name=\"country\">\n  <option value=\"us\">USA</option>\n  <option value=\"ca\">Canada</option>\n</select>"));
    html!("option", "An option within a select element.", "option", None);
    html!("textarea", "A multi-line plain-text editing control.", "textarea", Some("<textarea rows=\"4\" cols=\"50\">Default text</textarea>"));
    html!("br", "A line break within text.", "br", None);
    html!("hr", "A thematic break / horizontal rule.", "hr", None);
    html!("strong", "Strong importance (bold) text.", "strong", Some("<strong>Warning:</strong> This is important."));
    html!("em", "Emphasis (italic) text.", "em", Some("This is <em>really</em> important."));
    html!("code", "An inline code fragment.", "code", Some("Use the <code>print()</code> function."));
    html!("pre", "Preformatted text block (preserves whitespace).", "pre", Some("<pre>\n  function hello() {\n    console.log(\"hi\");\n  }\n</pre>"));
    html!("blockquote", "A block quotation from another source.", "blockquote", Some("<blockquote cite=\"https://source.com\">\n  Quoted text here.\n</blockquote>"));
    html!("cite", "The title of a cited work.", "cite", None);
    html!("abbr", "An abbreviation with an optional expansion.", "abbr", Some("<abbr title=\"Hypertext Markup Language\">HTML</abbr>"));
    html!("time", "A machine-readable date/time value.", "time", Some("<time datetime=\"2024-01-01\">January 1</time>"));
    html!("small", "Side-comments and small print.", "small", None);
    html!("sub", "Subscript text.", "sub", None);
    html!("sup", "Superscript text.", "sup", None);
    html!("dl", "A description list.", "dl", None);
    html!("dt", "A term in a description list.", "dt", None);
    html!("dd", "The description of a term.", "dd", None);
    html!("figure", "A self-contained figure (image, diagram, etc.).", "figure", None);
    html!("figcaption", "A caption for a figure element.", "figcaption", None);
    html!("details", "A disclosure widget that can be toggled open/closed.", "details", None);
    html!("summary", "A summary/caption for a details element.", "summary", None);
    html!("dialog", "A modal or non-modal dialog box.", "dialog", None);
    html!("progress", "A progress bar indicating completion.", "progress", Some("<progress value=\"70\" max=\"100\">70%</progress>"));
    html!("meter", "A scalar measurement within a known range.", "meter", None);
    html!("fieldset", "Groups related form controls.", "fieldset", None);
    html!("legend", "A caption for a fieldset.", "legend", None);

    // ── CSS Properties ────────────────────────────────────────────────────
    macro_rules! css {
        ($name:expr, $desc:expr, $url:expr, $ex:expr) => {
            m.insert($name, MdnEntry {
                category: MdnCategory::Css,
                title: $name,
                description: $desc,
                url: concat!("https://developer.mozilla.org/en-US/docs/Web/CSS/", $url),
                example: $ex,
            });
        };
    }
    css!("display", "Sets the display behavior of an element (block, inline, flex, grid, etc.).", "display", Some("display: flex;\ndisplay: none;\ndisplay: grid;"));
    css!("position", "Sets the positioning method for an element.", "position", Some("position: relative;\nposition: absolute;\nposition: fixed;\nposition: sticky;"));
    css!("color", "Sets the text color of an element.", "color", Some("color: #333;\ncolor: rgb(255, 0, 0);\ncolor: var(--primary);"));
    css!("background", "Shorthand for background-color, -image, -repeat, -position, -size.", "background", Some("background: #fff url(bg.png) no-repeat center/cover;"));
    css!("background-color", "Sets the background color of an element.", "background-color", Some("background-color: #f0f0f0;\nbackground-color: transparent;"));
    css!("background-image", "Sets one or more background images.", "background-image", Some("background-image: url('image.png');\nbackground-image: linear-gradient(...);"));
    css!("margin", "Sets the outer spacing on all four sides.", "margin", Some("margin: 0 auto;\nmargin: 10px 20px;\nmargin-top: 1rem;"));
    css!("padding", "Sets the inner spacing on all four sides.", "padding", Some("padding: 1rem;\npadding: 10px 20px;\npadding-left: 0;"));
    css!("border", "Shorthand for border-width, -style, and -color.", "border", Some("border: 1px solid #ccc;\nborder: none;\nborder-top: 2px dashed red;"));
    css!("width", "Sets the width of an element.", "width", Some("width: 100%;\nwidth: 300px;\nwidth: auto;\nwidth: fit-content;"));
    css!("height", "Sets the height of an element.", "height", Some("height: 100vh;\nheight: auto;\nmin-height: 200px;"));
    css!("max-width", "Sets the maximum width of an element.", "max-width", Some("max-width: 1200px;\nmax-width: 100%;"));
    css!("min-height", "Sets the minimum height of an element.", "min-height", None);
    css!("font-size", "Sets the size of the font.", "font-size", Some("font-size: 16px;\nfont-size: 1.2rem;\nfont-size: large;"));
    css!("font-weight", "Sets the weight (boldness) of the font.", "font-weight", Some("font-weight: bold;\nfont-weight: 400;\nfont-weight: 700;"));
    css!("font-family", "Specifies the font(s) for an element.", "font-family", Some("font-family: 'Inter', sans-serif;\nfont-family: monospace;"));
    css!("font-style", "Sets whether text is italic, oblique, or normal.", "font-style", Some("font-style: italic;\nfont-style: normal;"));
    css!("text-align", "Sets the horizontal alignment of inline content.", "text-align", Some("text-align: center;\ntext-align: left;\ntext-align: justify;"));
    css!("text-decoration", "Sets the decoration of text (underline, overline, line-through).", "text-decoration", Some("text-decoration: none;\ntext-decoration: underline wavy red;"));
    css!("text-transform", "Controls the capitalization of text.", "text-transform", Some("text-transform: uppercase;\ntext-transform: capitalize;"));
    css!("line-height", "Sets the height of each line of text.", "line-height", Some("line-height: 1.5;\nline-height: 24px;"));
    css!("letter-spacing", "Sets the spacing between characters.", "letter-spacing", Some("letter-spacing: 0.05em;"));
    css!("overflow", "Controls what happens to content that overflows its box.", "overflow", Some("overflow: hidden;\noverflow: auto;\noverflow: scroll;"));
    css!("overflow-x", "Controls horizontal overflow behavior.", "overflow-x", None);
    css!("overflow-y", "Controls vertical overflow behavior.", "overflow-y", None);
    css!("opacity", "Sets the transparency of an element (0–1).", "opacity", Some("opacity: 0.5;\nopacity: 1;"));
    css!("visibility", "Shows or hides an element without removing it from layout.", "visibility", Some("visibility: hidden;\nvisibility: visible;"));
    css!("cursor", "Sets the mouse cursor when hovering over an element.", "cursor", Some("cursor: pointer;\ncursor: not-allowed;\ncursor: grab;"));
    css!("z-index", "Sets the stack order of positioned elements.", "z-index", Some("z-index: 100;\nz-index: -1;"));
    css!("box-shadow", "Adds shadow effects around an element's frame.", "box-shadow", Some("box-shadow: 2px 2px 5px rgba(0,0,0,0.3);\nbox-shadow: none;"));
    css!("text-shadow", "Adds shadow effects to text.", "text-shadow", Some("text-shadow: 1px 1px 2px black;"));
    css!("border-radius", "Rounds the corners of an element's border box.", "border-radius", Some("border-radius: 8px;\nborder-radius: 50%;"));
    css!("transition", "Shorthand for transition-property, -duration, -timing, -delay.", "transition", Some("transition: all 0.3s ease;\ntransition: opacity 200ms;"));
    css!("transform", "Applies a 2D or 3D transformation to an element.", "transform", Some("transform: rotate(45deg);\ntransform: scale(1.2);\ntransform: translateX(10px);"));
    css!("animation", "Shorthand for all animation properties.", "animation", Some("animation: slideIn 0.5s ease forwards;\n@keyframes slideIn { from { opacity: 0; } to { opacity: 1; } }"));
    css!("flex", "Shorthand for flex-grow, flex-shrink, flex-basis.", "flex", Some("flex: 1;\nflex: 0 0 auto;"));
    css!("flex-direction", "Sets the direction of flex items (row, column, etc.).", "flex-direction", Some("flex-direction: row;\nflex-direction: column;"));
    css!("flex-wrap", "Controls whether flex items wrap onto multiple lines.", "flex-wrap", Some("flex-wrap: wrap;\nflex-wrap: nowrap;"));
    css!("justify-content", "Aligns flex items along the main axis.", "justify-content", Some("justify-content: center;\njustify-content: space-between;\njustify-content: flex-end;"));
    css!("align-items", "Aligns flex items along the cross axis.", "align-items", Some("align-items: center;\nalign-items: stretch;\nalign-items: flex-start;"));
    css!("align-content", "Aligns flex lines when there's extra space on cross axis.", "align-content", Some("align-content: center;\nalign-content: space-around;"));
    css!("gap", "Sets the spacing between rows and columns in grid/flex.", "gap", Some("gap: 1rem;\ngap: 8px 16px;"));
    css!("row-gap", "Sets the vertical gap in grid/flex layouts.", "row-gap", None);
    css!("column-gap", "Sets the horizontal gap in grid/flex layouts.", "column-gap", None);
    css!("grid-template-columns", "Defines the columns of a grid layout.", "grid-template-columns", Some("grid-template-columns: repeat(3, 1fr);\ngrid-template-columns: 200px auto 200px;"));
    css!("grid-template-rows", "Defines the rows of a grid layout.", "grid-template-rows", None);
    css!("grid-column", "Shorthand for grid-column-start / grid-column-end.", "grid-column", Some("grid-column: 1 / 3;\ngrid-column: span 2;"));
    css!("grid-row", "Shorthand for grid-row-start / grid-row-end.", "grid-row", None);
    css!("grid-area", "A shorthand for grid-row-start, grid-column-start, grid-row-end, grid-column-end.", "grid-area", Some("grid-area: header;\ngrid-area: 1 / 1 / 3 / 3;"));
    css!("list-style", "Shorthand for list-style-type, -position, -image.", "list-style", Some("list-style: none;\nlist-style: square inside;"));
    css!("object-fit", "Sets how an image/video fits within its box.", "object-fit", Some("object-fit: cover;\nobject-fit: contain;"));
    css!("filter", "Applies graphical effects like blur, contrast, grayscale.", "filter", Some("filter: blur(4px);\nfilter: grayscale(100%);\nfilter: brightness(1.2);"));
    css!("clip-path", "Clips an element to a basic shape or SVG path.", "clip-path", Some("clip-path: circle(50%);\nclip-path: polygon(0 0, 100% 0, 50% 100%);"));
    css!("pointer-events", "Controls whether an element responds to pointer events.", "pointer-events", Some("pointer-events: none;\npointer-events: auto;"));
    css!("user-select", "Controls whether text can be selected by the user.", "user-select", Some("user-select: none;\nuser-select: text;"));
    css!("scroll-behavior", "Controls smooth scrolling behavior.", "scroll-behavior", Some("scroll-behavior: smooth;\nscroll-behavior: auto;"));
    css!("resize", "Controls whether an element is resizable by the user.", "resize", Some("resize: both;\nresize: vertical;"));
    css!("outline", "A shorthand for outline-width, -style, -color (accessibility focus).", "outline", Some("outline: 2px solid blue;\noutline: none;"));
    css!("box-sizing", "Controls how width/height are calculated (content-box vs border-box).", "box-sizing", Some("box-sizing: border-box;\nbox-sizing: content-box;"));
    css!("float", "Places an element to the left or right of its container.", "float", Some("float: left;\nfloat: right;\nfloat: none;"));
    css!("clear", "Sets whether an element can be next to floated elements.", "clear", Some("clear: both;\nclear: left;"));
    css!("white-space", "Controls how whitespace and line breaks are handled.", "white-space", Some("white-space: nowrap;\nwhite-space: pre-wrap;"));
    css!("word-break", "Controls how words break at line endings.", "word-break", Some("word-break: break-all;\nword-break: keep-all;"));
    css!("content", "Used with ::before / ::after pseudo-elements to insert content.", "content", Some("content: \"\";\ncontent: attr(data-label);"));
    css!("var", "References a custom CSS property (CSS variable).", "var", Some("color: var(--primary-color, #333);"));

    // ── HTML Attributes ───────────────────────────────────────────────────
    macro_rules! attr {
        ($name:expr, $desc:expr, $url:expr) => {
            m.insert($name, MdnEntry {
                category: MdnCategory::Html,
                title: concat!($name, " (attribute)"),
                description: $desc,
                url: concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", $url),
                example: None,
            });
        };
    }
    attr!("class", "Assigns one or more class names to an element for CSS/styling.", "class");
    attr!("id", "Assigns a unique identifier to an element.", "id");
    attr!("href", "The URL of a linked resource (used in <a>, <link>).", "href");
    attr!("src", "The URL of an embedded resource (used in <img>, <script>, <video>).", "src");
    attr!("alt", "Alternative text for an image when it cannot be displayed.", "alt");
    attr!("type", "Specifies the type of an input or script element.", "type");
    attr!("value", "The current value of a form control.", "value");
    attr!("name", "The name of a form control, used for form submission.", "name");
    attr!("placeholder", "A hint text shown inside an input before user entry.", "placeholder");
    attr!("disabled", "When present, disables the element.", "disabled");
    attr!("readonly", "When present, makes the element read-only.", "readonly");
    attr!("required", "When present, the element must be filled in before form submission.", "required");
    attr!("checked", "When present, a checkbox/radio button is pre-selected.", "checked");
    attr!("selected", "When present, an <option> is pre-selected.", "selected");
    attr!("autocomplete", "Controls whether form input values are auto-completed by the browser.", "autocomplete");
    attr!("target", "Where to open a link (_self, _blank, _parent, _top).", "target");
    attr!("rel", "The relationship of a linked resource to the current document.", "rel");
    attr!("data-*", "Custom data attributes for storing private information.", "data-*");
    attr!("aria-*", "Accessible Rich Internet Applications attributes for accessibility.", "aria-*");
    attr!("role", "Defines the ARIA role of an element for accessibility.", "role");
    attr!("style", "Inline CSS styles for an element.", "style");
    attr!("tabindex", "Controls the tab order of focusable elements.", "tabindex");
    attr!("title", "Advisory information shown as a tooltip on hover.", "title");
    attr!("lang", "Specifies the language of an element's content.", "lang");
    attr!("dir", "The text direction (ltr, rtl) of an element.", "dir");
    attr!("hidden", "A boolean attribute indicating the element is not relevant.", "hidden");
    attr!("draggable", "Indicates whether the element can be dragged.", "draggable");
    attr!("contenteditable", "Indicates whether the element's content is editable.", "contenteditable");
    attr!("spellcheck", "Indicates whether spell checking is enabled for the element.", "spellcheck");
    attr!("loading", "Controls lazy loading for images/iframes (lazy, eager).", "loading");
    attr!("crossorigin", "Configures CORS requests for images, scripts, fonts.", "crossorigin");
    attr!("referrerpolicy", "Sets the referrer policy for fetches initiated by the element.", "referrerpolicy");
    attr!("sizes", "Specifies the sizes of icons for different viewports.", "sizes");

    // ── JavaScript ────────────────────────────────────────────────────────
    macro_rules! js {
        ($name:expr, $desc:expr, $url:expr, $ex:expr) => {
            m.insert($name, MdnEntry {
                category: MdnCategory::JavaScript,
                title: $name,
                description: $desc,
                url: concat!("https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/", $url),
                example: $ex,
            });
        };
    }
    js!("console", "Provides access to the browser's debugging console.", "Global_Objects/console", Some("console.log('Hello');\nconsole.error('Oops');\nconsole.table(data);"));
    js!("console.log", "Logs a message to the console.", "Global_Objects/console/log", Some("console.log('Hello, World!');"));
    js!("console.error", "Logs an error message to the console.", "Global_Objects/console/error", None);
    js!("console.warn", "Logs a warning message to the console.", "Global_Objects/console/warn", None);
    js!("console.table", "Displays tabular data as a table in the console.", "Global_Objects/console/table", None);
    js!("document", "The root DOM object representing the current HTML document.", "Global_Objects/document", Some("document.getElementById('app');\ndocument.querySelector('.class');"));
    js!("window", "The browser window object — the global scope for client-side JS.", "Global_Objects/window", Some("window.innerWidth;\nwindow.location.href;\nwindow.alert('Hi');"));
    js!("Array", "A globally available array-like object for storing ordered collections.", "Global_Objects/Array", Some("const arr = [1, 2, 3];\narr.map(x => x * 2);\narr.filter(x => x > 1);"));
    js!("Object", "The base class for all JavaScript objects.", "Global_Objects/Object", Some("Object.keys(obj);\nObject.values(obj);\nObject.entries(obj);"));
    js!("String", "A globally available object for string manipulation.", "Global_Objects/String", Some("\"hello\".toUpperCase();\n\"hello\".includes('ell');\n\"hello\".slice(1, 3);"));
    js!("Number", "A wrapper object for numeric values.", "Global_Objects/Number", Some("Number.parseInt('42');\nNumber.isNaN(val);"));
    js!("Math", "A built-in object with mathematical constants and functions.", "Global_Objects/Math", Some("Math.random();\nMath.floor(4.7);\nMath.max(1, 5, 3);"));
    js!("Date", "A built-in object for working with dates and times.", "Global_Objects/Date", Some("const d = new Date();\nd.toISOString();\nd.getFullYear();"));
    js!("JSON", "A built-in object for parsing and serializing JSON.", "Global_Objects/JSON", Some("JSON.parse(text);\nJSON.stringify(obj);"));
    js!("Promise", "Represents the eventual completion (or failure) of an async operation.", "Global_Objects/Promise", Some("fetch(url)\n  .then(res => res.json())\n  .catch(err => console.error(err));"));
    js!("fetch", "Initiates an HTTP(S) request and returns a Promise.", "Global_Objects/fetch", Some("const res = await fetch('/api/data');\nconst data = await res.json();"));
    js!("Map", "A collection of key-value pairs with insertion-order memory.", "Global_Objects/Map", Some("const m = new Map();\nm.set('key', 'value');\nm.get('key');"));
    js!("Set", "A collection of unique values of any type.", "Global_Objects/Set", Some("const s = new Set([1,2,3]);\ns.has(2);\ns.add(4);"));
    js!("Symbol", "A unique and immutable primitive value, often used as object keys.", "Global_Objects/Symbol", None);
    js!("Error", "A base object for runtime errors.", "Global_Objects/Error", Some("throw new Error('Something went wrong');\ntry { ... } catch (e) { ... }"));
    js!("RegExp", "A regular expression object for pattern matching.", "Global_Objects/RegExp", Some("/pattern/flags;\nnew RegExp('pattern', 'g');\nstr.match(/pattern/);"));
    js!("parseInt", "Parses a string argument and returns an integer.", "Global_Objects/parseInt", Some("parseInt('42', 10);\nparseInt('0xFF', 16);"));
    js!("parseFloat", "Parses a string argument and returns a floating-point number.", "Global_Objects/parseFloat", None);
    js!("isNaN", "Determines whether a value is NaN (not-a-number).", "Global_Objects/isNaN", None);
    js!("typeof", "Returns a string indicating the type of an unevaluated operand.", "Operators/typeof", Some("typeof 42;        // 'number'\ntypeof 'hello';   // 'string'\ntypeof undefined; // 'undefined'"));
    js!("instanceof", "Tests whether an object has a given constructor in its prototype chain.", "Operators/instanceof", Some("arr instanceof Array;\nerr instanceof Error;"));
    js!("new", "Creates an instance of a user-defined object type or built-in object.", "Operators/new", Some("const d = new Date();\nconst m = new Map();"));
    js!("this", "Refers to the execution context of the current function or scope.", "Operators/this", None);
    js!("function", "Declares a function with the specified parameters and body.", "Statements/function", Some("function greet(name) {\n  return `Hello, ${name}!`;\n}"));
    js!("const", "Declares a block-scoped, read-only named constant.", "Statements/const", Some("const PI = 3.14159;\nconst items = [1, 2, 3];"));
    js!("let", "Declares a block-scoped local variable.", "Statements/let", Some("let count = 0;\nif (true) { let x = 10; }"));
    js!("var", "Declares a function-scoped variable.", "Statements/var", Some("var name = 'old style';"));
    js!("if", "Executes a block of code if a specified condition is truthy.", "Statements/if...else", Some("if (x > 0) {\n  console.log('positive');\n} else {\n  console.log('not positive');\n}"));
    js!("else", "Executes a block when the if condition is falsy.", "Statements/if...else", None);
    js!("else if", "Specifies a new condition if the first condition is falsy.", "Statements/if...else", None);
    js!("for", "Creates a loop with optional initialization, condition, and increment.", "Statements/for", Some("for (let i = 0; i < 10; i++) {\n  console.log(i);\n}"));
    js!("for...of", "Iterates over iterable objects (arrays, strings, maps, etc.).", "Statements/for...of", Some("for (const item of items) {\n  console.log(item);\n}"));
    js!("for...in", "Iterates over the enumerable properties of an object.", "Statements/for...in", None);
    js!("while", "Creates a loop that executes as long as a condition is truthy.", "Statements/while", Some("while (i < 10) {\n  i++;\n}"));
    js!("do...while", "Creates a loop that executes once before checking the condition.", "Statements/do...while", None);
    js!("switch", "Evaluates an expression and matches its value to a case clause.", "Statements/switch", Some("switch (color) {\n  case 'red': break;\n  default: break;\n}"));
    js!("case", "A clause in a switch statement.", "Statements/switch", None);
    js!("break", "Terminates the current loop or switch statement.", "Statements/break", None);
    js!("continue", "Skips to the next iteration of a loop.", "Statements/continue", None);
    js!("return", "Exits a function and optionally returns a value.", "Statements/return", Some("return result;\nreturn;"));
    js!("throw", "Throws a user-defined exception.", "Statements/throw", Some("throw new Error('fail');"));
    js!("try", "Defines a block of code to test for errors.", "Statements/try...catch", Some("try {\n  riskyOperation();\n} catch (err) {\n  console.error(err);\n}"));
    js!("catch", "Defines a block to handle errors from a try block.", "Statements/try...catch", None);
    js!("finally", "Defines a block that runs after try/catch regardless of the outcome.", "Statements/try...catch", None);
    js!("async", "Declares an asynchronous function that returns a Promise.", "Statements/async_function", Some("async function loadData() {\n  const res = await fetch(url);\n  return res.json();\n}"));
    js!("await", "Pauses execution of an async function until a Promise resolves.", "Operators/await", Some("const data = await fetch(url);"));
    js!("class", "Declares a class with constructor, methods, and static members.", "Statements/class", Some("class Rectangle {\n  constructor(w, h) {\n    this.w = w; this.h = h;\n  }\n  area() { return this.w * this.h; }\n}"));
    js!("extends", "Creates a subclass that inherits from a parent class.", "Statements/class", None);
    js!("super", "References the parent class constructor or methods.", "Operators/super", None);
    js!("import", "Imports bindings from an external module.", "Statements/import", Some("import { useState } from 'react';\nimport * as utils from './utils.js';"));
    js!("export", "Exports bindings from a module for use in other modules.", "Statements/export", Some("export function helper() {}\nexport default class App {}"));
    js!("from", "Specifies the module path in an import statement.", "Statements/import", None);
    js!("typeof", "Returns a string describing the type of a value.", "Operators/typeof", None);
    js!("delete", "Removes a property from an object.", "Operators/delete", None);
    js!("void", "Evaluates an expression and returns undefined.", "Operators/void", None);
    js!("null", "A primitive value representing the intentional absence of any object value.", "Global_Objects/null", None);
    js!("undefined", "A primitive value automatically assigned to variables that have not been initialized.", "Global_Objects/undefined", None);
    js!("true", "The boolean true value.", "Global_Objects/Boolean", None);
    js!("false", "The boolean false value.", "Global_Objects/Boolean", None);
    js!("NaN", "A special value representing Not-A-Number.", "Global_Objects/NaN", None);
    js!("Infinity", "A numeric value representing infinity.", "Global_Objects/Infinity", None);
    js!("globalThis", "The global object, available in all JavaScript environments.", "Global_Objects/globalThis", None);
    js!("localStorage", "A web storage API for persisting key-value data in the browser.", "Global_Objects/localStorage", Some("localStorage.setItem('key', 'value');\nconst val = localStorage.getItem('key');\nlocalStorage.removeItem('key');"));
    js!("sessionStorage", "A web storage API for session-scoped key-value data.", "Global_Objects/sessionStorage", None);
    js!("innerHTML", "Gets or sets the HTML content inside an element.", "Global_Objects/Element/innerHTML", Some("el.innerHTML = '<p>New content</p>';"));
    js!("innerText", "Gets or sets the rendered text content of an element.", "Global_Objects/HTMLElement/innerText", None);
    js!("textContent", "Gets or sets the text content of an element and its descendants.", "Global_Objects/Node/textContent", None);
    js!("addEventListener", "Registers an event handler on a target (element, document, window).", "Global_Objects/EventTarget/addEventListener", Some("element.addEventListener('click', (e) => {\n  console.log('clicked');\n});"));
    js!("removeEventListener", "Removes an event handler previously registered with addEventListener.", "Global_Objects/EventTarget/removeEventListener", None);
    js!("querySelector", "Returns the first element matching a CSS selector.", "Global_Objects/Document/querySelector", Some("document.querySelector('.my-class');\ndocument.querySelector('#my-id');"));
    js!("querySelectorAll", "Returns a static NodeList of all elements matching a CSS selector.", "Global_Objects/Document/querySelectorAll", Some("document.querySelectorAll('li');\ndocument.querySelectorAll('[data-type]');"));
    js!("getElementById", "Returns the element with the specified ID.", "Global_Objects/Document/getElementById", Some("document.getElementById('app');"));
    js!("getElementsByClassName", "Returns a live HTMLCollection of elements with a given class.", "Global_Objects/Document/getElementsByClassName", None);
    js!("getElementsByTagName", "Returns a live HTMLCollection of elements with a given tag name.", "Global_Objects/Document/getElementsByTagName", None);
    js!("createElement", "Creates a new HTML element with the given tag name.", "Global_Objects/Document/createElement", Some("const div = document.createElement('div');"));
    js!("setAttribute", "Sets the value of an attribute on an element.", "Global_Objects/Element/setAttribute", None);
    js!("getAttribute", "Returns the value of a specified attribute on an element.", "Global_Objects/Element/getAttribute", None);
    js!("classList", "A read-only DOMTokenList that provides methods for managing element classes.", "Global_Objects/Element/classList", Some("el.classList.add('active');\nel.classList.remove('hidden');\nel.classList.toggle('open');"));
    js!("style", "Gets or sets the inline style of an element.", "Global_Objects/HTMLElement/style", None);
    js!("setTimeout", "Calls a function after a specified delay (in milliseconds).", "Global_Objects/setTimeout", Some("setTimeout(() => {\n  console.log('Delayed');\n}, 1000);"));
    js!("setInterval", "Repeatedly calls a function with a fixed time delay between each call.", "Global_Objects/setInterval", Some("const id = setInterval(() => {\n  console.log('Tick');\n}, 1000);\nclearInterval(id);"));
    js!("clearTimeout", "Cancels a timeout established by setTimeout.", "Global_Objects/clearTimeout", None);
    js!("clearInterval", "Cancels a repeated action established by setInterval.", "Global_Objects/clearInterval", None);
    js!("requestAnimationFrame", "Schedules a function to run before the next repaint.", "Global_Objects/window/requestAnimationFrame", Some("requestAnimationFrame((timestamp) => {\n  // animate\n});"));
    js!("map", "Creates a new array populated with the results of calling a function on every element.", "Global_Objects/Array/map", Some("const doubled = [1,2,3].map(x => x * 2);"));
    js!("filter", "Creates a new array with elements that pass a test function.", "Global_Objects/Array/filter", Some("const evens = [1,2,3,4].filter(x => x % 2 === 0);"));
    js!("reduce", "Executes a reducer function on each element, producing a single value.", "Global_Objects/Array/reduce", Some("const sum = [1,2,3].reduce((acc, x) => acc + x, 0);"));
    js!("forEach", "Executes a provided function once for each array element.", "Global_Objects/Array/forEach", Some("arr.forEach((item, i) => console.log(i, item));"));
    js!("find", "Returns the first element that satisfies a testing function.", "Global_Objects/Array/find", Some("const found = [1,2,3].find(x => x > 1);"));
    js!("findIndex", "Returns the index of the first element that satisfies a testing function.", "Global_Objects/Array/findIndex", None);
    js!("some", "Tests whether at least one element passes a test function.", "Global_Objects/Array/some", Some("const hasBig = values.some(v => v > 100);"));
    js!("every", "Tests whether all elements pass a test function.", "Global_Objects/Array/every", None);
    js!("includes", "Determines whether an array or string contains a certain value.", "Global_Objects/Array/includes", Some("arr.includes(3);\nstr.includes('sub');"));
    js!("push", "Adds one or more elements to the end of an array and returns the new length.", "Global_Objects/Array/push", Some("arr.push(4);"));
    js!("pop", "Removes the last element from an array and returns it.", "Global_Objects/Array/pop", None);
    js!("shift", "Removes the first element from an array and returns it.", "Global_Objects/Array/shift", None);
    js!("unshift", "Adds one or more elements to the beginning of an array and returns the new length.", "Global_Objects/Array/unshift", None);
    js!("slice", "Returns a shallow copy of a portion of an array or string.", "Global_Objects/Array/slice", Some("arr.slice(1, 3);\nstr.slice(0, 5);"));
    js!("splice", "Changes an array by removing, replacing, or adding elements in place.", "Global_Objects/Array/splice", Some("arr.splice(1, 1);\narr.splice(0, 0, 'new');"));
    js!("sort", "Sorts the elements of an array in place and returns the array.", "Global_Objects/Array/sort", Some("arr.sort();\narr.sort((a, b) => a - b);"));
    js!("join", "Joins all elements of an array into a string.", "Global_Objects/Array/join", Some("arr.join(', ');"));
    js!("concat", "Merges two or more arrays into a new array.", "Global_Objects/Array/concat", None);
    js!("indexOf", "Returns the first index at which a given element can be found.", "Global_Objects/Array/indexOf", None);
    js!("lastIndexOf", "Returns the last index at which a given element can be found.", "Global_Objects/Array/lastIndexOf", None);
    js!("flat", "Returns a new array with all sub-array elements concatenated into it recursively.", "Global_Objects/Array/flat", None);
    js!("flatMap", "Maps each element then flattens the result into a new array.", "Global_Objects/Array/flatMap", None);
    js!("keys", "Returns a new iterator of an array's keys (indices).", "Global_Objects/Array/keys", None);
    js!("values", "Returns a new iterator of an array's values.", "Global_Objects/Array/values", None);
    js!("entries", "Returns an iterator of an array's index-value pairs.", "Global_Objects/Array/entries", None);
    js!("from", "Creates a new Array from an iterable or array-like object.", "Global_Objects/Array/from", Some("Array.from('hello');\nArray.from(set);"));
    js!("isArray", "Determines whether a value is an Array.", "Global_Objects/Array/isArray", None);
    js!("hasOwnProperty", "Returns a boolean indicating whether an object has the specified property.", "Global_Objects/Object/hasOwnProperty", None);
    js!("assign", "Copies own enumerable properties from source objects to a target object.", "Global_Objects/Object/assign", Some("Object.assign(target, source);"));
    js!("defineProperty", "Defines a new or modifies an existing property on an object.", "Global_Objects/Object/defineProperty", None);
    js!("freeze", "Freezes an object, preventing new properties and changes.", "Global_Objects/Object/freeze", None);
    js!("seal", "Seals an object, preventing additions/deletions but allowing value changes.", "Global_Objects/Object/seal", None);
    js!("create", "Creates a new object with a specified prototype.", "Global_Objects/Object/create", None);
    js!("setPrototypeOf", "Sets the prototype of a specified object.", "Global_Objects/Object/setPrototypeOf", None);
    js!("getPrototypeOf", "Returns the prototype of a specified object.", "Global_Objects/Object/getPrototypeOf", None);
    js!("toUpperCase", "Converts a string to uppercase.", "Global_Objects/String/toUpperCase", None);
    js!("toLowerCase", "Converts a string to lowercase.", "Global_Objects/String/toLowerCase", None);
    js!("trim", "Removes whitespace from both ends of a string.", "Global_Objects/String/trim", None);
    js!("split", "Divides a string into an ordered list of substrings.", "Global_Objects/String/split", Some("str.split(',');"));
    js!("replace", "Returns a new string with some or all matches of a pattern replaced.", "Global_Objects/String/replace", Some("str.replace('old', 'new');\nstr.replace(/old/g, 'new');"));
    js!("replaceAll", "Returns a new string with all matches of a pattern replaced.", "Global_Objects/String/replaceAll", None);
    js!("startsWith", "Determines whether a string begins with a given substring.", "Global_Objects/String/startsWith", None);
    js!("endsWith", "Determines whether a string ends with a given substring.", "Global_Objects/String/endsWith", None);
    js!("match", "Retrieves the result of matching a string against a regular expression.", "Global_Objects/String/match", None);
    js!("search", "Executes a search for a match between a regex and a string.", "Global_Objects/String/search", None);
    js!("charAt", "Returns the character at a specified index in a string.", "Global_Objects/String/charAt", None);
    js!("charCodeAt", "Returns the Unicode value of the character at a given index.", "Global_Objects/String/charCodeAt", None);
    js!("padStart", "Pads the current string from the start with a given string.", "Global_Objects/String/padStart", None);
    js!("padEnd", "Pads the current string from the end with a given string.", "Global_Objects/String/padEnd", None);
    js!("repeat", "Returns a new string with the specified number of copies.", "Global_Objects/String/repeat", None);
    js!("length", "Returns the length of a string or the number of elements in an array.", "Global_Objects/String/length", None);
    js!("toFixed", "Formats a number using fixed-point notation.", "Global_Objects/Number/toFixed", Some("(3.14159).toFixed(2); // '3.14'"));
    js!("toExponential", "Returns a string representing a number in exponential notation.", "Global_Objects/Number/toExponential", None);
    js!("toPrecision", "Returns a string representing a number with a specified precision.", "Global_Objects/Number/toPrecision", None);
    js!("isInteger", "Determines whether a value is an integer.", "Global_Objects/Number/isInteger", None);
    js!("isFinite", "Determines whether a value is a finite number.", "Global_Objects/isFinite", None);
    js!("encodeURI", "Encodes a URI by escaping special characters.", "Global_Objects/encodeURI", None);
    js!("encodeURIComponent", "Encodes a URI component by escaping special characters.", "Global_Objects/encodeURIComponent", None);
    js!("decodeURI", "Decodes a URI created by encodeURI.", "Global_Objects/decodeURI", None);
    js!("decodeURIComponent", "Decodes a URI component created by encodeURIComponent.", "Global_Objects/decodeURIComponent", None);
    js!("Math.random", "Returns a random number between 0 (inclusive) and 1 (exclusive).", "Global_Objects/Math/random", Some("Math.random();\nMath.floor(Math.random() * 10);"));
    js!("Math.floor", "Returns the largest integer less than or equal to a number.", "Global_Objects/Math/floor", None);
    js!("Math.ceil", "Returns the smallest integer greater than or equal to a number.", "Global_Objects/Math/ceil", None);
    js!("Math.round", "Returns the value of a number rounded to the nearest integer.", "Global_Objects/Math/round", None);
    js!("Math.max", "Returns the largest of zero or more numbers.", "Global_Objects/Math/max", None);
    js!("Math.min", "Returns the smallest of zero or more numbers.", "Global_Objects/Math/min", None);
    js!("Math.abs", "Returns the absolute value of a number.", "Global_Objects/Math/abs", None);
    js!("Math.pow", "Returns base raised to exponent power.", "Global_Objects/Math/pow", Some("Math.pow(2, 3); // 8"));
    js!("Math.sqrt", "Returns the positive square root of a number.", "Global_Objects/Math/sqrt", None);
    js!("Math.sin", "Returns the sine of an angle (in radians).", "Global_Objects/Math/sin", None);
    js!("Math.cos", "Returns the cosine of an angle (in radians).", "Global_Objects/Math/cos", None);
    js!("Math.PI", "The mathematical constant π (pi).", "Global_Objects/Math/PI", None);
    js!("Math.E", "The mathematical constant e (Euler's number).", "Global_Objects/Math/E", None);
    js!("Promise.all", "Takes an iterable of promises and returns a single Promise that resolves when all resolve.", "Global_Objects/Promise/all", Some("const [a, b] = await Promise.all([p1, p2]);"));
    js!("Promise.allSettled", "Returns a Promise that resolves after all promises settle (resolve or reject).", "Global_Objects/Promise/allSettled", None);
    js!("Promise.race", "Returns a Promise that resolves/rejects as soon as one promise settles.", "Global_Objects/Promise/race", None);
    js!("Promise.any", "Returns a Promise that resolves as soon as one promise resolves.", "Global_Objects/Promise/any", None);
    js!("Promise.resolve", "Returns a Promise object that is resolved with a given value.", "Global_Objects/Promise/resolve", None);
    js!("Promise.reject", "Returns a Promise object that is rejected with a given reason.", "Global_Objects/Promise/reject", None);
    js!("then", "Attaches fulfillment/rejection handlers to a Promise.", "Global_Objects/Promise/then", Some("promise.then(value => {}, error => {});"));
    js!("catch", "Attaches a rejection handler to a Promise.", "Global_Objects/Promise/catch", Some("promise.catch(err => console.error(err));"));
    js!("finally", "Attaches a handler that runs when a Promise settles.", "Global_Objects/Promise/finally", None);
    js!("resolve", "Returns a Promise that is resolved with a given value.", "Global_Objects/Promise/resolve", None);
    js!("reject", "Returns a Promise that is rejected with a given reason.", "Global_Objects/Promise/reject", None);

    // ── CSS At-rules and functions ────────────────────────────────────────
    css!("import", "Imports external stylesheets into a CSS file.", "@import", Some("@import url('reset.css');\n@import 'typography.css';"));
    css!("media", "Applies styles based on media queries (screen size, device type, etc.).", "@media", Some("@media (max-width: 768px) {\n  body { font-size: 14px; }\n}"));
    css!("keyframes", "Defines a multi-step animation by specifying keyframe styles.", "@keyframes", Some("@keyframes slide-in {\n  from { transform: translateX(-100%); }\n  to { transform: translateX(0); }\n}"));
    css!("font-face", "Defines a custom font family that can be used in font-family declarations.", "@font-face", Some("@font-face {\n  font-family: 'MyFont';\n  src: url('myfont.woff2');\n}"));
    css!("supports", "Applies styles only if the browser supports a specific CSS feature.", "@supports", Some("@supports (display: grid) {\n  .grid { display: grid; }\n}"));
    css!("container", "Defines a containment context for container queries.", "@container", Some("@container (min-width: 400px) {\n  .card { flex-direction: row; }\n}"));
    css!("layer", "Defines a named cascade layer to control declaration order.", "@layer", Some("@layer base, components;\n@layer base {\n  body { margin: 0; }\n}"));
    css!("scale", "Scales an element in 2D space (part of transform).", "transform-function/scale", Some("transform: scale(1.5);\ntransform: scaleX(2);"));
    css!("rotate", "Rotates an element by a given angle.", "transform-function/rotate", Some("transform: rotate(45deg);\ntransform: rotateX(180deg);"));
    css!("translate", "Moves an element along the X and/or Y axes.", "transform-function/translate", Some("transform: translate(10px, 20px);\ntransform: translateY(-50%);"));
    css!("skew", "Skews an element along the X and/or Y axes.", "transform-function/skew", Some("transform: skew(10deg, 5deg);"));
    css!("calc", "Performs a calculation to be used as a property value.", "calc", Some("width: calc(100% - 40px);\nfont-size: calc(1rem + 0.5vw);"));
    css!("min", "Selects the minimum value from a comma-separated list.", "min", Some("width: min(100%, 800px);"));
    css!("max", "Selects the maximum value from a comma-separated list.", "max", Some("width: max(300px, 50%);"));
    css!("clamp", "Clamps a value between an upper and lower bound.", "clamp", Some("font-size: clamp(1rem, 2.5vw, 2rem);"));
    css!("env", "Accesses environment variables (safe-area-inset-top, etc.).", "env", Some("padding-top: env(safe-area-inset-top);"));
    css!("url", "References a resource file (image, font, etc.).", "url", Some("background-image: url('bg.png');"));

    // ── CSS Pseudo-classes ────────────────────────────────────────────────
    css!("hover", "Applies when the user hovers over an element with a pointing device.", ":hover", Some("button:hover {\n  background: #555;\n}"));
    css!("focus", "Applies when an element has received focus via keyboard or mouse.", ":focus", Some("input:focus {\n  border-color: blue;\n  outline: none;\n}"));
    css!("active", "Applies while an element is being activated (clicked).", ":active", None);
    css!("visited", "Applies to links that have been visited by the user.", ":visited", None);
    css!("link", "Applies to links that have not been visited yet.", ":link", None);
    css!("first-child", "Applies to the first child element among its siblings.", ":first-child", Some("li:first-child { font-weight: bold; }"));
    css!("last-child", "Applies to the last child element among its siblings.", ":last-child", Some("li:last-child { border-bottom: none; }"));
    css!("nth-child", "Applies to elements based on their position in a group of siblings.", ":nth-child", Some("tr:nth-child(even) { background: #f5f5f5; }"));
    css!("nth-of-type", "Applies to elements based on their position among siblings of the same type.", ":nth-of-type", None);
    css!("not", "Selects elements that do NOT match a given selector.", ":not", Some("input:not([type='submit']) {\n  border: 1px solid #ccc;\n}"));
    css!("is", "Takes a selector list and matches any element that matches any selector.", ":is", None);
    css!("where", "Similar to :is but with zero specificity.", ":where", None);
    css!("has", "Selects an element if it contains a specific child/descendant.", ":has", Some("figure:has(figcaption) {\n  border: 1px solid #ddd;\n}"));
    css!("placeholder-shown", "Applies to inputs showing placeholder text.", ":placeholder-shown", None);
    css!("focus-visible", "Applies when focus is received via keyboard navigation.", ":focus-visible", None);
    css!("focus-within", "Applies when an element or any of its descendants have focus.", ":focus-within", None);
    css!("disabled", "Applies to disabled form elements.", ":disabled", Some("input:disabled {\n  background: #eee;\n  cursor: not-allowed;\n}"));
    css!("enabled", "Applies to enabled form elements.", ":enabled", None);
    css!("checked", "Applies to checked radio buttons, checkboxes, and option elements.", ":checked", None);
    css!("required", "Applies to form elements with the required attribute.", ":required", None);
    css!("optional", "Applies to form elements without the required attribute.", ":optional", None);
    css!("valid", "Applies to form elements with valid content.", ":valid", None);
    css!("invalid", "Applies to form elements with invalid content.", ":invalid", None);
    css!("target", "Applies to the element whose ID matches the current URL fragment.", ":target", None);
    css!("root", "Applies to the root element of the document (:root = html).", ":root", None);
    css!("empty", "Applies to elements with no children (including text nodes).", ":empty", None);
    css!("first-of-type", "Applies to the first element of its type among siblings.", ":first-of-type", None);
    css!("last-of-type", "Applies to the last element of its type among siblings.", ":last-of-type", None);
    css!("only-child", "Applies to an element that is the only child of its parent.", ":only-child", None);
    css!("only-of-type", "Applies to an element that is the only one of its type among siblings.", ":only-of-type", None);

    // ── CSS Pseudo-elements ────────────────────────────────────────────────
    css!("before", "Creates a pseudo-element that is the first child of the selected element.", "::before", Some(".alert::before {\n  content: \"⚠️ \";\n}"));
    css!("after", "Creates a pseudo-element that is the last child of the selected element.", "::after", Some(".tooltip::after {\n  content: attr(data-tip);\n}"));
    css!("first-line", "Applies styles to the first line of a block-level element.", "::first-line", None);
    css!("first-letter", "Applies styles to the first letter of a block-level element.", "::first-letter", None);
    css!("selection", "Applies styles to the portion of the text selected by the user.", "::selection", None);
    css!("placeholder", "Styles the placeholder text of an input or textarea.", "::placeholder", None);
    css!("marker", "Styles the list marker of a list item (bullet/number).", "::marker", None);
    css!("backdrop", "Styles the backdrop of a dialog or fullscreen element.", "::backdrop", None);
    css!("scrollbar", "Styles the scrollbar of an element (non-standard, but widely supported).", "::-webkit-scrollbar", None);
    css!("file-selector-button", "Styles the button of an <input type=\"file\">.", "::file-selector-button", None);

    m
});

pub fn lookup_mdn(keyword: &str) -> Option<&'static MdnEntry> {
    MDN_REFERENCES.get(keyword)
}

pub fn lookup_mdn_for_language(keyword: &str, language: &str) -> Option<&'static MdnEntry> {
    let entry = MDN_REFERENCES.get(keyword)?;
    let cat = match language.to_lowercase().as_str() {
        "html" | "htm" => matches!(entry.category, MdnCategory::Html),
        "css" | "scss" | "less" => matches!(entry.category, MdnCategory::Css),
        "javascript" | "js" | "jsx" | "tsx" | "typescript" | "ts" => {
            matches!(entry.category, MdnCategory::JavaScript)
        }
        _ => true,
    };
    if cat { Some(entry) } else { None }
}
