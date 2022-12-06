import { useStore } from "@nanostores/react";
import { engines, enginesList } from "../stores/engine";
import { CampaignCell } from "./CampaignCell";

export const CampaignList = ({ children }) => {
  const $list = useStore(enginesList);
  const $engines = useStore(engines);

  return (
    <>
      <ul className={"list"}>
        {$list.map((item) => (
          <CampaignCell item={$engines[item]} key={item} path={item} />
        ))}
      </ul>
      <div className={$list.length ? "hidden" : "full-center"}>{children}</div>
    </>
  );
};
