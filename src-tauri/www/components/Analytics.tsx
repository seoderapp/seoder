import "chartist/dist/index.css";
import "../styles/chart.css";

import { BarChart } from "chartist";
import { useEffect, Fragment } from "react";
import { useStore } from "@nanostores/react";
import { engines, enginesList } from "../stores/engine";

const options = {
  showPoint: false,
  lineSmooth: true,
  axisX: {
    showGrid: false,
    showLabel: true,
  },
  axisY: {
    offset: 40,
    labelInterpolationFnc: function (value) {
      return value;
    },
  },
  plugins: [],
};

const legend = [
  { name: "Valids", color: "rgb(16 185 129)" },
  { name: "Invalids", color: "rgb(234 179 8)" },
  { name: "Errors", color: "rgb(220 38 38)" },
];

export const Analytics = () => {
  const $campaignList = useStore(enginesList);
  const $campaigns = useStore(engines);

  useEffect(() => {
    const valids = [];
    const invalids = [];
    const errors = [];

    $campaignList.forEach((item) => {
      const engine = $campaigns[item];

      if (engine) {
        valids.push(engine.urls.size);
        invalids.push(engine.invalidUrls.size);
        errors.push(engine.errorUrls.size);
      }
    });

    const data = {
      labels: $campaignList,
      series: [
        { name: "Valids", data: valids },
        { name: "Invalids", data: invalids },
        { name: "Errors", data: errors },
      ],
    };

    new BarChart("#chart", data, options);
  }, [$campaignList, $campaigns]);

  return (
    <div style={{ minWidth: "10rem" }}>
      <div style={{ height: "0.5rem" }}></div>
      <div className="flex row gap ph gutter-xl center">
        {legend.map(({ name, color }) => {
          return (
            <Fragment key={name}>
              <div className="flex row gap center">
                <div style={{ fontSize: "0.75rem", color: "rgb(30, 30, 30)" }}>
                  {name}
                </div>
                <span style={{ backgroundColor: color, height: 8, width: 8 }} />
              </div>
            </Fragment>
          );
        })}
      </div>
      <div id={"chart"}></div>
    </div>
  );
};
