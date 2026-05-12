import { PartialBlock } from "@blocknote/core";
import { filterSuggestionItems } from "@blocknote/core/extensions";
import "@blocknote/core/fonts/inter.css";
import { zh } from "@blocknote/core/locales";
import { BlockNoteView } from "@blocknote/mantine";
import "@blocknote/mantine/style.css";
import {
  FormattingToolbar,
  FormattingToolbarController,
  getDefaultReactSlashMenuItems,
  getFormattingToolbarItems,
  SuggestionMenuController,
  useCreateBlockNote,
} from "@blocknote/react";
import {
  AIExtension,
  AIMenuController,
  AIToolbarButton,
  ClientSideTransport,
  getAISlashMenuItems,
} from "@blocknote/xl-ai";
import { zh as aiZh } from "@blocknote/xl-ai/locales";
import "@blocknote/xl-ai/style.css";
import { createOpenAICompatible } from "@ai-sdk/openai-compatible";
import { useCallback, useEffect, useRef } from "react";
import "./blocknote-editor.css";

const MOUSE_OUTSIDE_CLASS = "bn-editor-mouse-outside";

const deepseekModel = createOpenAICompatible({
  name: "deepseek",
  apiKey: "sk-8cc48d1ef9794ee9ae0a8f71d2995b4b",
  baseURL: "https://api.deepseek.com",
})("deepseek-chat");

interface BlockNoteEditorProps {
  initialContent?: string;
  placeholder?: string;
  onChange?: (markdown: string) => void;
}

export default function BlockNoteEditorComponent({
  initialContent,
  placeholder,
  onChange,
}: BlockNoteEditorProps) {
  const isInitialLoad = useRef(true);

  const editor = useCreateBlockNote({
    dictionary: {
      ...zh,
      ai: aiZh,
      placeholders: {
        ...zh.placeholders,
        default: placeholder || "开始写作...",
        heading: placeholder || "标题",
      },
    },
    extensions: [
      AIExtension({
        transport: new ClientSideTransport({
          model: deepseekModel,
        }),
      }),
    ],
    initialContent: [
      {
        type: "paragraph",
      } as PartialBlock,
    ],
  });

  useEffect(() => {
    if (initialContent && editor && isInitialLoad.current) {
      isInitialLoad.current = false;
      const blocks = editor.tryParseMarkdownToBlocks(initialContent);
      if (blocks.length > 0) {
        editor.replaceBlocks(editor.document, blocks);
      }
    }
  }, [editor]);

  const handleChange = useCallback(() => {
    if (onChange && editor) {
      const markdown = editor.blocksToMarkdownLossy(editor.document);
      onChange(markdown);
    }
  }, [onChange, editor]);

  const wrapperRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      const wrapperElement = wrapperRef.current;
      const portalElement = editor?.portalElement;
      if (!wrapperElement) return;

      const target = e.target as HTMLElement;
      const isInsideWrapper = wrapperElement.contains(target);
      const isInsidePortal = portalElement?.contains(target) ?? false;

      if (isInsideWrapper || isInsidePortal) {
        document.body.classList.remove(MOUSE_OUTSIDE_CLASS);
      } else {
        document.body.classList.add(MOUSE_OUTSIDE_CLASS);
      }
    };

    document.addEventListener("mousemove", handleMouseMove, true);
    return () => {
      document.removeEventListener("mousemove", handleMouseMove, true);
      document.body.classList.remove(MOUSE_OUTSIDE_CLASS);
    };
  }, [editor]);

  return (
    <div ref={wrapperRef} className="bn-editor-custom" style={{ flex: 1, display: "flex", flexDirection: "column", overflow: "hidden" }}>
      <BlockNoteView
        editor={editor}
        formattingToolbar={false}
        slashMenu={false}
        onChange={handleChange}
        style={{ flex: 1 }}
      >
        <AIMenuController />
        <FormattingToolbarController
          formattingToolbar={() => (
            <FormattingToolbar>
              {...getFormattingToolbarItems()}
              <AIToolbarButton />
            </FormattingToolbar>
          )}
        />
        <SuggestionMenuController
          triggerCharacter="/"
          getItems={async (query) =>
            filterSuggestionItems(
              [
                ...getDefaultReactSlashMenuItems(editor),
                ...getAISlashMenuItems(editor),
              ],
              query,
            )
          }
        />
      </BlockNoteView>
    </div>
  );
}
