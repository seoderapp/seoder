import { useRef, useEffect, useState } from "react";
import { useStore } from "@nanostores/react";
import { errorLogs, invalidLogs, validLogs } from "../stores/engine";

export interface Props {
  id?: string;
  emptyId?: string;
  logs?: Set<string | unknown>;
  scrolling?: boolean;
}

const LogItem = ({ children }) => <li>{children}</li>;

// scroll to bottom hook
const useScrolling = ({ divRef, list, scrolling, atom }) => {
  const [scrollEnabled, setScrolling] = useState<boolean>(scrolling);

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

  // todo: control scroll state from parent
  useEffect(() => {
    setScrolling(scrolling);
  }, [scrolling]);

  const onToggleScrolling = () => setScrolling((v) => !v);

  return {
    onToggleScrolling,
    scrollEnabled,
  };
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
      <button onClick={onToggleScrolling} className={"scroll-base"}>
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
export function Log({ id, emptyId, logs, scrolling }: Props) {
  const list = Array.from(logs?.keys() ?? []);
  const divRef = useRef(null);
  const empty = !list.length;

  const { scrollEnabled, onToggleScrolling } = useScrolling({
    divRef,
    list: list,
    scrolling: scrolling,
    atom: validLogs,
  });

  return (
    <div className="log">
      <ul id={id} role={"list"}>
        <EmptyCell empty={empty} emptyId={emptyId} />
        {list.map((item: string) => (
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

export function LogValid({ id, emptyId, scrolling }: Props) {
  const $list = useStore(validLogs);
  const divRef = useRef(null);

  const { scrollEnabled, onToggleScrolling } = useScrolling({
    divRef,
    list: $list,
    scrolling: scrolling,
    atom: validLogs,
  });

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

  const { scrollEnabled, onToggleScrolling } = useScrolling({
    divRef,
    list: $list,
    scrolling,
    atom: invalidLogs,
  });

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

  const { scrollEnabled, onToggleScrolling } = useScrolling({
    divRef,
    list: $list,
    scrolling: scrolling,
    atom: errorLogs,
  });

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
