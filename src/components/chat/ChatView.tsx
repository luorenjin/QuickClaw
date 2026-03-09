import { useState, useRef, useEffect } from "react";
import { api } from "../../hooks/api";
import { ClawConfig, ChatMessage } from "../../types";
import "./ChatView.css";

interface Props {
  config: ClawConfig;
}

let msgIdCounter = 0;
const newId = () => String(++msgIdCounter);

export default function ChatView({ config }: Props) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState("");
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  // Auto-scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const send = async () => {
    const text = input.trim();
    if (!text || sending) return;

    const userMsg: ChatMessage = { id: newId(), role: "user", content: text };
    setMessages((prev) => [...prev, userMsg]);
    setInput("");
    setSending(true);
    setError(null);

    try {
      const reply = await api.sendChatMessage(config, messages, text);
      const assistantMsg: ChatMessage = {
        id: newId(),
        role: "assistant",
        content: reply,
      };
      setMessages((prev) => [...prev, assistantMsg]);
    } catch (err) {
      setError(String(err));
    } finally {
      setSending(false);
      setTimeout(() => inputRef.current?.focus(), 50);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  const clearHistory = () => {
    if (window.confirm("确定要清空所有对话记录吗？")) {
      setMessages([]);
    }
  };

  return (
    <div className="chat">
      {/* Header */}
      <div className="chat-header">
        <div className="chat-header-info">
          <span className="chat-avatar">🦀</span>
          <div>
            <div className="chat-name">
              {config.claw_name || "Claw"}
            </div>
            <div className="chat-role text-muted text-sm">
              {config.claw_role || "AI 助手"} · {config.server_url}
            </div>
          </div>
        </div>
        <button className="btn-ghost chat-clear-btn" onClick={clearHistory}>
          🗑 清空记录
        </button>
      </div>

      {/* Messages */}
      <div className="chat-messages">
        {messages.length === 0 && (
          <div className="chat-empty">
            <span className="chat-empty-icon">🦀</span>
            <p>你好！我是 {config.claw_name || "Claw"}，{config.claw_role || "有什么我能帮助你的吗？"}</p>
            <p className="text-sm text-muted">按 Enter 发送消息，Shift+Enter 换行</p>
          </div>
        )}
        {messages.map((msg) => (
          <div key={msg.id} className={`chat-msg chat-msg-${msg.role}`}>
            <div className="chat-msg-bubble">
              <pre className="chat-msg-content">{msg.content}</pre>
            </div>
          </div>
        ))}
        {sending && (
          <div className="chat-msg chat-msg-assistant">
            <div className="chat-msg-bubble chat-msg-typing">
              <span className="typing-dot" />
              <span className="typing-dot" />
              <span className="typing-dot" />
            </div>
          </div>
        )}
        {error && (
          <div className="chat-error">
            ⚠ {error}
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Input area */}
      <div className="chat-input-area">
        <textarea
          ref={inputRef}
          className="chat-input"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={`发消息给 ${config.claw_name || "Claw"}… (Enter 发送 / Shift+Enter 换行)`}
          rows={1}
          disabled={sending}
        />
        <button
          className="btn-primary chat-send-btn"
          onClick={send}
          disabled={!input.trim() || sending}
        >
          {sending ? <span className="spinner" /> : "发送"}
        </button>
      </div>
    </div>
  );
}
