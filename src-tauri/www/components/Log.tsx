import { useStore } from "@nanostores/react";
import { FC } from "react";
import { errorLogs, validLogs } from "../stores/engine";

export interface Props {
  id?: string;
  emptyId?: string;
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
  const list = useStore(errorLogs);

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
  const list = useStore(validLogs);

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
