import { useStore } from "@nanostores/react";
import { engines, enginesList } from "../stores/engine";
import { CampaignCell } from "./CampaignCell";

export const CampaignList = ({ children }) => {
  const $list = useStore(enginesList);
  const engineItems = engines.get();

  return (
    <>
      <ul className={"list"}>
        {$list.map((item) => (
          <CampaignCell item={engineItems[item]} key={item} path={item} />
        ))}
      </ul>
      <div className={$list.length ? "hidden" : "full-center"}>{children}</div>
    </>
  );
};
