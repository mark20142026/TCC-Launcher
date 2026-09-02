// Shared Telegram + OpenRouter helpers for the TCC bot functions.

export const TG_API = 'https://api.telegram.org';
export const OPENROUTER_API = 'https://openrouter.ai/api/v1/chat/completions';

// Download URL kept in sync with the CI versionless release asset.
export const DOWNLOAD_URL =
  'https://github.com/pnbx/TCC-Launcher/releases/latest/download/TCC-Launcher-latest-setup.exe';

const SYSTEM_PROMPT = [
  'تو دستیار رسمی کامیونیتی TCC هستی: یک سرور ماینکرفت PvP ایرانی با لانچر اختصاصی (TCC Launcher).',
  'قواعد پاسخ: کوتاه، دوستانه، فارسیِ ساده. ایموجی کم ولی به‌جا.',
  'اطلاعات کلیدی: آدرس سرور p1.pfmc.ir و پورت 19155 (نسخه 1.21.4)، لانچر با آپدیت خودکار، نصب از طریق سایت رسمی.',
  'اگه سوال فنی درباره‌ی کرش یا باگ بود، راهنمایی اولیه بده و بگو جزئیات (اسکرین‌شات/لاگ) رو بفرسته.',
  'سوالات خارج از موضوع رو هم مؤدبانه و کوتاه جواب بده.',
].join('\n');

export function telegramToken() {
  return process.env.TELEGRAM_BOT_TOKEN || '';
}

export function telegramChat() {
  return process.env.TELEGRAM_CHAT_ID || '';
}

export function openrouterKeys() {
  return (process.env.OPENROUTER_KEYS || '')
    .split(',')
    .map((k) => k.trim())
    .filter(Boolean);
}

export async function tg(method, body) {
  const res = await fetch(`${TG_API}/bot${telegramToken()}/${method}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  return res.json().catch(() => null);
}

let rotation = 0;

async function openrouterChat(model, messages) {
  const keys = openrouterKeys();
  if (keys.length === 0) return null;

  // Try every key once, starting from the rotating offset, so rate limits
  // spread across all of them.
  for (let i = 0; i < keys.length; i++) {
    const key = keys[(rotation + i) % keys.length];
    try {
      const res = await fetch(OPENROUTER_API, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${key}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ model, messages, max_tokens: 400, temperature: 0.6 }),
      });
      if (res.status === 429 || res.status >= 500) continue;
      const data = await res.json().catch(() => null);
      const text = data?.choices?.[0]?.message?.content?.trim();
      if (text) {
        rotation = (rotation + i + 1) % keys.length;
        return text;
      }
    } catch {}
  }
  return null;
}

export async function askAI(question) {
  const model = process.env.OPENROUTER_MODEL || 'deepseek/deepseek-chat-v3-0324:free';
  return openrouterChat(model, [
    { role: 'system', content: SYSTEM_PROMPT },
    { role: 'user', content: question },
  ]);
}
