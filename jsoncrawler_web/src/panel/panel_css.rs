pub const RAW_CSS: &'static str = r#"
<style>
  body {
    background: #19222c;
    color: #fff;
    font-family: system-ui, Helvetica;
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
  background-color: var(--bs-card-bg);
  border: var(--bs-card-border-width) solid var(--bs-card-border-color);
  border-radius: var(--bs-card-border-radius);
  display: flex;
  flex-direction: column;
  min-height: 155px;
  min-width: 0;
  position: relative;
}
.card-body {
  color: var(--bs-card-color);
  flex: 1 1 auto;
  padding: var(--bs-card-spacer-y) var(--bs-card-spacer-x);
}

#campaign-list {
  list-style: none;
  padding: 0;
  margin: 0;
  border: 1px solid #ccc;
  border-radius: 4px;
}

#campaign-list > .campaign-item {
  padding: 9px 12px;
  font-weight: semi-bold;
  font-size: 1.1rem;
}

#campaign-list > .campaign-item:not(:first-child) {
  border-top: 1px solid #ccc;
}

.seperator {
  height: 4px;
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
  background-color: #222e3c;
  border: 1px solid #ccc;
  border-radius: 0.2rem;
  color: #fff;
  font-size: .875rem;
  font-weight: 400;
  line-height: 1.5;
  padding: 0.3rem 0.85rem;
  transition: border-color .15s ease-in-out,box-shadow .15s ease-in-out;
}

</style>

"#;
