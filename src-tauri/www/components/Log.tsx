import {
  useRef,
  useEffect,
  useState,
  memo,
  FC,
  PropsWithChildren,
} from "react";
import { useStore } from "@nanostores/react";
import {
  errorLogs,
  invalidLogs,
  selectedContacts,
  validLogs,
} from "../stores/engine";
import { WritableAtom } from "nanostores";
import { contactsModalData, modalStore, ModalType } from "../stores/app";

export interface Props {
  id?: string;
  emptyId?: string;
  logs?: string[];
  focused?: boolean;
  atom?: WritableAtom<string[]>;
}

export interface LogProps {
  contacts?: string[];
}

const LogItemStored = ({ children, contacts }) => {
  const onContactClick = () => {
    contactsModalData.set({ domain: children, contacts });
    modalStore.set(ModalType.CONTACTS);
  };

  return (
    <li className="flex row cell">
      <div className="flex-1">{children}</div>
      <button
        onClick={onContactClick}
        style={{ padding: "0.1rem 0.8rem", borderRadius: "2rem" }}
      >
        Contact
      </button>
    </li>
  );
};

const LogItemWrapper: FC<PropsWithChildren<LogProps>> = ({
  children,
  contacts,
}) => {
  if (contacts && contacts?.length) {
    return <LogItemStored contacts={contacts}>{children}</LogItemStored>;
  }
  return <li className="cell">{children}</li>;
};

const LogItem = memo(LogItemWrapper);

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

// log panel with selected contact state
export function LogS({ id, emptyId, focused, atom }: Props) {
  const $list = useStore(atom);
  const $contacts = useStore(selectedContacts);

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
            <LogItem
              key={item}
              contacts={
                $contacts?.has(item) ? $contacts.get(item).contacts : []
              }
            >
              {item}
            </LogItem>
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
  return <LogS id={id} emptyId={emptyId} focused={focused} atom={validLogs} />;
}

// invalid list
export function LogInvalid({ id, emptyId, focused }: Props) {
  return <Log id={id} emptyId={emptyId} focused={focused} atom={invalidLogs} />;
}

// error list
export function LogErrors({ id, emptyId, focused }: Props) {
  return <Log id={id} emptyId={emptyId} focused={focused} atom={errorLogs} />;
}
