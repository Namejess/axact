import { h, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

// Initialize htm with Preact
const html = htm.bind(h);

function App(props) {
  return html`
    <div>
      ${props.cpus.map((cpu) => {
        return html` <div>${cpu.toFixed(2)}% usage</div> `;
      })}
    </div>
  `;
}

document.addEventListener("DOMContentLoaded", () => {
  let i = 0;

  setInterval(async () => {
    let response = await fetch("/api/cpus");
    if (response.status !== 200) {
      throw new Error(`Failed to fetch status: ${response.status}`);
    }

    let json = await response.json();

    i = i + 1;
    // document.body.textContent = JSON.stringify(json, null, 2);
    const app = h("pre", null, JSON.stringify(json, null, 2));

    render(html`<${App} cpus=${json}></${App}>`, document.body);
  }, 1000);
});
