import { useStore } from "@nanostores/react";
import { useMemo } from "react";
import { engines, enginesList } from "../stores/engine";
import { CampaignCell } from "./CampaignCell";

export const CampaignList = ({ children }) => {
  const $list = useStore(enginesList);
  const $engines = useStore(engines);

  // memo items for improved rendering
  const items = useMemo(() => {
    return (
      <>
        {$list.map((item) => (
          <CampaignCell item={$engines[item]} key={item} path={item} />
        ))}
      </>
    );
  }, [$list, $engines]);

  return (
    <>
      <ul className={"list"}>{items}</ul>
      <div className={$list.length ? "hidden" : "full-center"}>{children}</div>
    </>
  );
};
