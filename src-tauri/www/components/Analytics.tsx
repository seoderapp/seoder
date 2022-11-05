import "chartist/dist/index.css";
import "../styles/chart.css";

import { BarChart } from "chartist";
import { useEffect } from "react";
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
};

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
        valids, // valids
        invalids, // invalids
        errors, // errors
      ],
    };

    new BarChart("#chart", data, options);
  }, [$campaignList, $campaigns]);

  return (
    <div style={{ minWidth: "10rem" }}>
      <div style={{ height: "0.5rem" }}></div>
      <div id={"chart"}></div>
    </div>
  );
};
