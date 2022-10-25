import { FC, useRef, useEffect, useState } from "react";
import { useStore } from "@nanostores/react";
import { errorLogs, invalidLogs, validLogs } from "../stores/engine";

export interface Props {
  id?: string;
  emptyId?: string;
  logs?: Set<string | unknown>;
  scrolling?: boolean;
}

const LogItem: FC = ({ children }) => <li>{children}</li>;

const scrollStyles = {
  padding: "0.4rem",
  background: "transparent",
  color: "#fff",
  textAlign: "center",
  display: "flex",
  margin: "auto",
};

// scroll to bottom hook
const useScrolling = ({ divRef, list, scrolling, atom }) => {
  useEffect(() => {
    const listener = atom.listen((v) => {
      if (scrolling && v.length > list.length) {
        divRef?.current?.scrollIntoView();
      }
    });
    return () => {
      listener();
    };
  }, [list, scrolling]);
};

// scroll to bottom of component
function ScrollButton({ empty, divRef, scrollEnabled, onToggleScrolling }) {
  const style = !empty
    ? {
        padding: 0,
      }
    : {
        display: "none",
      };
  return (
    <li className={"empty-log"} style={style} ref={divRef}>
      <button onClick={onToggleScrolling} style={scrollStyles}>
        {scrollEnabled ? "Stop Scrolling" : "Stick to Bottom"}
      </button>
    </li>
  );
}

const EmptyCell = ({ emptyId, empty }) => {
  return (
    <li
      id={emptyId}
      className={empty ? "block gutter-t" : "hidden"}
      style={{ textAlign: "center" }}
    >
      Ready to run
    </li>
  );
};

// log panel
export function Log({ id, emptyId, logs }: Props) {
  const list = Array.from(logs?.keys() ?? []);

  return (
    <div className="log">
      <ul id={id} role={"list"}>
        {!list.length ? (
          <li id={emptyId} className={"block gutter-t"}>
            Ready to run
          </li>
        ) : null}
        {list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
      </ul>
    </div>
  );
}

export function LogValid({ id, emptyId, scrolling }: Props) {
  const $list = useStore(validLogs);
  const divRef = useRef(null);
  const [scrollEnabled, setScrolling] = useState<boolean>(scrolling);

  useScrolling({
    divRef,
    list: $list,
    scrolling: scrollEnabled,
    atom: validLogs,
  });

  // todo: control scroll state from parent
  useEffect(() => {
    setScrolling(scrolling);
  }, [scrolling]);

  const onToggleScrolling = () => setScrolling((v) => !v);

  const empty = !$list.length;

  return (
    <div className="log">
      <ul id={id} role={"list"}>
        <EmptyCell empty={empty} emptyId={emptyId} />

        {$list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
        <ScrollButton
          empty={empty}
          scrollEnabled={scrollEnabled}
          divRef={divRef}
          onToggleScrolling={onToggleScrolling}
        />
      </ul>
    </div>
  );
}

export function LogInvalid({ id, emptyId, scrolling }: Props) {
  const $list = useStore(invalidLogs);
  const divRef = useRef(null);
  const [scrollEnabled, setScrolling] = useState<boolean>(scrolling);

  useScrolling({
    divRef,
    list: $list,
    scrolling: scrollEnabled,
    atom: invalidLogs,
  });

  // todo: control scroll state from parent
  useEffect(() => {
    setScrolling(scrolling);
  }, [scrolling]);

  const onToggleScrolling = () => setScrolling((v) => !v);

  const empty = !$list.length;

  return (
    <div className="log">
      <ul id={id} role={"list"}>
        <EmptyCell empty={empty} emptyId={emptyId} />

        {$list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
        <ScrollButton
          empty={empty}
          scrollEnabled={scrollEnabled}
          divRef={divRef}
          onToggleScrolling={onToggleScrolling}
        />
      </ul>
    </div>
  );
}

export function LogErrors({ id, emptyId, scrolling }: Props) {
  const $list = useStore(errorLogs);
  const divRef = useRef(null);
  const [scrollEnabled, setScrolling] = useState<boolean>(scrolling);

  useScrolling({
    divRef,
    list: $list,
    scrolling: scrollEnabled,
    atom: errorLogs,
  });

  useEffect(() => {
    setScrolling(scrolling);
  }, [scrolling]);

  const onToggleScrolling = () => setScrolling((v) => !v);

  const empty = !$list.length;

  return (
    <div className="log">
      <ul id={id} role={"list"}>
        <EmptyCell empty={empty} emptyId={emptyId} />

        {$list.map((item: string) => (
          <LogItem key={item}>{item}</LogItem>
        ))}
        <ScrollButton
          empty={empty}
          scrollEnabled={scrollEnabled}
          divRef={divRef}
          onToggleScrolling={onToggleScrolling}
        />
      </ul>
    </div>
  );
}
