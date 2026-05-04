import { PartialBlock } from "@blocknote/core";
import { filterSuggestionItems } from "@blocknote/core/extensions";
import "@blocknote/core/fonts/inter.css";
import { en } from "@blocknote/core/locales";
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
import { en as aiEn } from "@blocknote/xl-ai/locales";
import "@blocknote/xl-ai/style.css";
import { createOpenAICompatible } from "@ai-sdk/openai-compatible";
import { useCallback, useEffect, useRef } from "react";

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
      ...en,
      ai: aiEn,
      placeholders: {
        ...en.placeholders,
        default: placeholder || "Start writing...",
        heading: placeholder || "Heading",
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

  return (
    <div style={{ flex: 1, display: "flex", flexDirection: "column", overflow: "hidden" }}>
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
