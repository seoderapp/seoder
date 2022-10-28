import { useRef, useEffect, useState } from "react";
import { useStore } from "@nanostores/react";
import { errorLogs, invalidLogs, validLogs } from "../stores/engine";
import { WritableAtom } from "nanostores";

export interface Props {
  id?: string;
  emptyId?: string;
  logs?: string[];
  focused?: boolean;
  atom?: WritableAtom<string[]>;
}

const LogItem = ({ children }) => <li>{children}</li>;

// scroll to bottom hook
const useScrolling = ({ divRef, list, focused, atom }) => {
  const [scrollEnabled, setScrolling] = useState<boolean>(focused);

  useEffect(() => {
    const listener = atom.listen((v) => {
      if (scrollEnabled && v.length > list.length) {
        divRef?.current?.scrollIntoView();
      }
    });
    return () => {
      listener();
    };
  }, [list, scrollEnabled]);

  // todo: control scroll state from parent
  useEffect(() => {
    setScrolling(focused);
  }, [focused]);

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

// empty panel display center
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
export function Log({ id, emptyId, focused, atom }: Props) {
  const $list = useStore(atom);
  const divRef = useRef(null);
  const empty = !$list.length;

  const { scrollEnabled, onToggleScrolling } = useScrolling({
    divRef,
    list: $list,
    focused,
    atom,
  });

  return (
    <div
      className={focused ? "" : "hidden"}
      role="tabpanel"
      aria-labelledby="tab-console"
    >
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
    </div>
  );
}

// valid list
export function LogValid({ id, emptyId, focused }: Props) {
  return <Log id={id} emptyId={emptyId} focused={focused} atom={validLogs} />;
}

// invalid list
export function LogInvalid({ id, emptyId, focused }: Props) {
  return <Log id={id} emptyId={emptyId} focused={focused} atom={invalidLogs} />;
}

// error list
export function LogErrors({ id, emptyId, focused }: Props) {
  return <Log id={id} emptyId={emptyId} focused={focused} atom={errorLogs} />;
}
