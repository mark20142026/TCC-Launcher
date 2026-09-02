import { askAI, cors, telegramChat, tg } from './_telegram.js';

let botUsernameCache = null;

async function botUsername() {
  if (botUsernameCache) return botUsernameCache;
  const me = await tg('getMe', {});
  botUsernameCache = me?.result?.username || '';
  return botUsernameCache;
}

// Replies when the bot is addressed: a direct reply to one of its messages,
// an @mention, the /ask command, or a private chat message.
function shouldRespond(message, username) {
  if (!username) return false;
  if (message.chat.type === 'private') return true;
  if (message.reply_to_message?.from?.username === username) return true;
  if (message.text.includes('@' + username)) return true;
  if (message.text.startsWith('/ask')) return true;
  return false;
}

async function handleUpdate(update) {
  const message = update.message || update.edited_message;
  if (!message?.text) return;

  // Optional lock: only ever talk in the configured chat.
  const allowed = telegramChat();
  if (allowed && String(message.chat.id) !== String(allowed)) return;

  const username = await botUsername();
  if (!shouldRespond(message, username)) return;

  const question = message.text
    .replace(new RegExp('@' + username, 'g'), '')
    .replace(/^\/ask\s*/, '')
    .trim();

  const answer = await askAI(question || message.text);

  await tg('sendMessage', {
    chat_id: message.chat.id,
    reply_to_message_id: message.message_id,
    text: answer || 'الان نتونستم جواب بدم؛ یه کم بعد دوباره امتحان کن 🙏',
  });
}

export default async function handler(req, res) {
  cors(res);

  if (req.method === 'GET') {
    return res.status(200).json({ ok: true, service: 'tcc-telegram-bot' });
  }
  if (req.method !== 'POST') return res.status(405).end();

  const update = typeof req.body === 'object' && req.body !== null
    ? req.body
    : JSON.parse(req.body || '{}');

  try {
    await handleUpdate(update);
  } catch (err) {
    console.error('telegram update failed:', err);
  }

  // Always 200: Telegram retries non-2xx webhooks aggressively.
  res.status(200).end();
}
