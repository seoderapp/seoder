import { useStore } from "@nanostores/react";
import { enginesList } from "../stores/engine";
import { CampaignCell } from "./CampaignCell";

export const CampaignList = ({ children }) => {
  const $list = useStore(enginesList);

  return (
    <>
      <ul className={"list"}>
        {$list.map((item) => (
          <CampaignCell item={item} key={item} path={item} />
        ))}
      </ul>
      <div className={$list.length ? "hidden" : "full-center"}>{children}</div>
    </>
  );
};
