pub const RAW_CSS: &'static str = r#"
<style>

  body {
    font-family: system-ui, Helvetica;
  }

  h2,h3,h4,h5,h6 {
    padding: 0 0 0.4rem 0;
    margin: 0;
  }

  h3 {
    padding-bottom: 0.55rem;
  }

  .gutter {
    padding-bottom: 0.18rem;
  }

  .card {
    --bs-card-spacer-y: 1.25rem;
    --bs-card-spacer-x: 1.25rem;
    --bs-card-title-spacer-y: 0.5rem;
    --bs-card-border-width: 0;
    --bs-card-border-color: transparent;
    --bs-card-border-radius: 0.25rem;
    --bs-card-inner-border-radius: 0.25rem;
    --bs-card-cap-padding-y: 1rem;
    --bs-card-cap-padding-x: 1.25rem;
    --bs-card-cap-bg: #222e3c;
    --bs-card-bg: #222e3c;
    --bs-card-img-overlay-padding: 1rem;
    --bs-card-group-margin: 12px;
    word-wrap: break-word;
    background-clip: border-box;
    border: var(--bs-card-border-width) solid var(--bs-card-border-color);
    border-radius: var(--bs-card-border-radius);
    display: flex;
    flex-direction: column;
    min-height: 100px;
    min-width: 0;
    position: relative;
  }

  .card-body {
    color: var(--bs-card-color);
    padding: var(--bs-card-spacer-y) var(--bs-card-spacer-x);
  }

  #campaign-list,#engine-list {
    list-style: none;
    padding: 0;
    margin: 0;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  #campaign-list > .campaign-item {
    padding: 0.44rem 0.8rem;
    font-weight: 300;
    font-size: 1.1rem;
    display: flex;
    flex: 1;
  }

  #campaign-list > .campaign-item:not(:first-child) {
    border-top: 1px solid #ccc;
  }

  #campaign-list li.campaign-item > div > div:nth-child(1) {
    padding-bottom: 0.2rem;
    font-weight: bold;
  }

  #campaign-list li.campaign-item > div > div:nth-child(2) {
    padding-bottom: 0.3rem;
  }

  #campaign-list li.campaign-item > div > div:nth-child(3) {
    padding-bottom: 0.22rem;
    font-size: 0.9rem;
  }

  #campaign-list li.campaign-item button {
    min-width: 65px;
    border: none;
    border-radius: 0.2rem;
    margin-left: 0.54rem;
    font-weight: bold;
    font-size: 1rem;
    padding: 0.45rem 0.7rem;
  }

  #campaign-list li.campaign-item button:hover {
    opacity: 0.7;
  }

  #campaign-list li.campaign-item button:focus {
    outline: 0.5px solid #ccc;
  }

  #campaign-list li.campaign-item button:nth-child(1) {
    color: #1b1b1b;
  }

  #campaign-list li.campaign-item button:nth-child(2) {
    background-color: #1b1b1b;
    color: #fff;
  }

  #engine-list > .engine-item {
    padding: 0.44rem 0.8rem;
    font-weight: 300;
    font-size: 1.1rem;
    display: flex;
    flex: 1;
  }

  #engine-list li.engine-item button:hover {
    opacity: 0.7;
  }

  #engine-list li.engine-item button {
    min-width: 65px;
    border: none;
    border-radius: 0.2rem;
    margin-left: 0.54rem;
    font-weight: bold;
    font-size: 1rem;
  }

  #engine-list li.engine-item button:nth-child(1) {
    border: 1px solid #0e1116;
    color: #0e1116;
    padding: 0.45rem 0.7rem;
  }

  #engine-list > .engine-item:not(:first-child) {
    border-top: 1px solid #ccc;
  }

  #engine-list li.engine-item > div:first-of-type {
    font-weight: bold;
    padding-bottom: 0.4rem;
  }

  .seperator {
    height: 0.5px;
    background-color: #d4d4d4;
  }

  .button {
    --bs-btn-padding-x: 0.85rem;
    --bs-btn-padding-y: 0.3rem;
    --bs-btn-font-family: ;
    --bs-btn-font-size: 0.875rem;
    --bs-btn-font-weight: 400;
    --bs-btn-line-height: 1.5;
    --bs-btn-color: #bdc0c5;
    --bs-btn-bg: transparent;
    --bs-btn-border-width: 1px;
    --bs-btn-border-color: #fff;
    --bs-btn-border-radius: 0.2rem;
    --bs-btn-box-shadow: inset 0 1px 0 rgba(34,46,60,.15),0 1px 1px hsla(0,0%,100%,.075);
    --bs-btn-disabled-opacity: 0.65;
    --bs-btn-focus-box-shadow: 0 0 0 0.2rem rgba(var(--bs-btn-focus-shadow-rgb),.5);
    background-color: var(--bs-btn-bg);
    border: var(--bs-btn-border-width) solid var(--bs-btn-border-color);
    border-radius: var(--bs-btn-border-radius);
    color: var(--bs-btn-color);
    cursor: pointer;
    display: inline-block;
    font-family: var(--bs-btn-font-family);
    font-size: var(--bs-btn-font-size);
    font-weight: var(--bs-btn-font-weight);
    line-height: var(--bs-btn-line-height);
    padding: var(--bs-btn-padding-y) var(--bs-btn-padding-x);
    text-align: center;
    transition: color .15s ease-in-out,background-color .15s ease-in-out,border-color .15s ease-in-out,box-shadow .15s ease-in-out;
    user-select: none;
    vertical-align: middle;
  }

  .btn-primary {
    --bs-btn-color: #222e3c;
    --bs-btn-bg: #3b7ddd;
    --bs-btn-border-color: #2f64b1;
    --bs-btn-hover-color: #fff;
    --bs-btn-hover-bg: #326abc;
    --bs-btn-hover-border-color: #2f64b1;
    --bs-btn-focus-shadow-rgb: 55,113,197;
    --bs-btn-active-color: #fff;
    --bs-btn-active-bg: #2f64b1;
    --bs-btn-active-border-color: #2c5ea6;
    --bs-btn-active-shadow: inset 0 3px 5px hsla(0,0%,100%,.125);
    --bs-btn-disabled-color: #222e3c;
    --bs-btn-disabled-bg: #3b7ddd;
    --bs-btn-disabled-border-color: #3b7ddd;
    color: #fff;
  }

  .form-control {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;
    background-clip: padding-box;
    border: 1px solid #ccc;
    border-radius: 0.2rem;
    font-size: .875rem;
    font-weight: 400;
    line-height: 1.5;
    padding: 0.3rem 0.85rem;
    transition: border-color .15s ease-in-out,box-shadow .15s ease-in-out;
  }

  .stats-box {
    border: 1px solid #ccc;
    padding: 0.5rem;
    font-size: 0.87rem;
    width: 260px;
    place-content: center;
    display: flex;
    flex-direction: column;
  }

  .cpu-box {
    border: 1px solid #ccc;
    padding: 0.5rem;
    width: 100px;
  }

  .tild {
    font-size: 0.84rem;
    padding-bottom: 0.4rem;
  }

  .box {
    padding: 0.3rem;
  }

  .bar {
    display: flex;
    flex: 1;
    justify-content: flex-end;
    align-items: center;
    padding-bottom: 0.45rem;
    padding-top: 0.45rem;
    flex-wrap: wrap-reverse;
  }

  .row {
    display: flex;
    padding: 0.5rem;
    flex-direction: row;
  }

  .stats-box .seperator {
    padding-bottom: 0.1rem;
  }

  .flex {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .center {
    place-content: center;
  }

  .text-center {
    text-align:center;
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  .ph {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  .stats-bar {
    font-size: 0.8rem;
    width: 180px;
    max-width: 180px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  #cpu-stats {
    width: 80px;
    height: 80px;
    background: rgba(30, 30, 30, 0.1);
    border-radius: 50%;
  }

  .frame {
    border: 1px solid #ccc;
    border-radius: 2px;
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
  }

  .flex-col {
    display: flex;
    flex-direction: column;
  }

  #cpu-stats-average {
    font-size: 0.8rem;
    font-weight: 300;
  }

  .stats-head {
    font-weight: bold;
    font-size: 0.8rem;
  }

  #uploadform button {
    background-color: transparent;
    border: 1px solid #0e1116;
    color: #0e1116;
  }
</style>

"#;
