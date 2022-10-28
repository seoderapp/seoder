import { useRef, useEffect, useState } from "react";
import { useStore } from "@nanostores/react";
import { errorLogs, invalidLogs, validLogs } from "../stores/engine";
import { WritableAtom } from "nanostores";

export interface Props {
  id?: string;
  emptyId?: string;
  logs?: string[];
  scrolling?: boolean;
  atom?: WritableAtom<string[]>;
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
export function Log({ id, emptyId, scrolling, atom }: Props) {
  const $list = useStore(atom);
  const divRef = useRef(null);
  const empty = !$list.length;

  const { scrollEnabled, onToggleScrolling } = useScrolling({
    divRef,
    list: $list,
    scrolling: scrolling,
    atom,
  });

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

export function LogValid({ id, emptyId, scrolling }: Props) {
  return (
    <Log id={id} emptyId={emptyId} scrolling={scrolling} atom={validLogs} />
  );
}

export function LogInvalid({ id, emptyId, scrolling }: Props) {
  return (
    <Log id={id} emptyId={emptyId} scrolling={scrolling} atom={invalidLogs} />
  );
}

export function LogErrors({ id, emptyId, scrolling }: Props) {
  return (
    <Log id={id} emptyId={emptyId} scrolling={scrolling} atom={errorLogs} />
  );
}
