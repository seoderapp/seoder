import { useStore } from "@nanostores/react";
import { FC } from "react";
import { selectedErrors, selectedValid } from "../stores/engine";

export interface Props {
  id: string;
  emptyId: string;
  logs?: Set<string | unknown>;
}

const LogItem: FC = ({ children }) => {
  return <li>{children}</li>;
};

// log panel
export function Log({ id, emptyId, logs }: Props) {
  const list = Array.from(logs?.keys() ?? []);

  return (
    <div className="log">
      {!list.length ? (
        <div id={emptyId} className={"block gutter-t"}>
          Ready to run
        </div>
      ) : null}
      <ul id={id} role={"list"}>
        {list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
      </ul>
    </div>
  );
}

// unused log errors single
export function LogErrors({ id, emptyId }: Props) {
  const logs = useStore(selectedErrors);
  const list = Array.from(logs?.keys() ?? []);

  return (
    <div className="log">
      {!list.length ? (
        <div id={emptyId} className={"block gutter-t"}>
          Ready to run
        </div>
      ) : null}
      <ul id={id} role={"list"}>
        {list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
      </ul>
    </div>
  );
}

// unused log valid panel single
export function LogValid({ id, emptyId }: Props) {
  const logs = useStore(selectedValid);
  const list = Array.from(logs?.keys() ?? []);

  return (
    <div className="log">
      {!list.length ? (
        <div id={emptyId} className={"block gutter-t"}>
          Ready to run
        </div>
      ) : null}
      <ul id={id} role={"list"}>
        {list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
      </ul>
    </div>
  );
}
