import { useStore } from "@nanostores/react";
import { FC } from "react";
import { engines, enginesList } from "../stores/engine";
import { CampaignCell } from "./CampaignCell";

export const CampaignList: FC = ({ children }) => {
  const $list = useStore(enginesList);
  const engineItems = engines.get();

  const emptyStyle = $list.length ? "hidden" : "block";

  return (
    <>
      <ul className={"list"}>
        {$list.map((item) => (
          <CampaignCell item={engineItems[item]} key={item} path={item} />
        ))}
      </ul>
      <div className={emptyStyle}>{children}</div>
    </>
  );
};
