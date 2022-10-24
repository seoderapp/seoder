import { useStore } from "@nanostores/react";
import { engines, enginesList } from "../stores/engine";
import { CampaignCell } from "./CampaignCell";

export const CampaignList = ({ children }: any) => {
  const list = useStore(enginesList);
  const engineItems = engines.get();

  // empty form css visiblity
  const emptyStyle = list.length ? "hidden" : "block";

  return (
    <>
      <ul className={"list"}>
        {list.map((item) => (
          <CampaignCell item={engineItems[item]} key={item} path={item} />
        ))}
      </ul>
      <div className={emptyStyle}>{children}</div>
    </>
  );
};
