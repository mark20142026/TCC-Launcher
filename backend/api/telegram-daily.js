import { cors, DOWNLOAD_URL, tg, telegramChat, telegramToken } from './_telegram.js';

// Rotating daily poll topics for the community chat.
const TOPICS = [
  {
    question: 'نسخه‌ی جدید لانچر TCC رو نصب کردید؟ چطور بود؟',
    options: ['نصب کردم، عالی بود 👍', 'نصب کردم، مشکل داشتم 🐛', 'هنوز نصب نکردم ⏳'],
  },
  {
    question: 'دوست دارید چه چیزی به سرور اضافه بشه؟',
    options: ['مپ/آرنای جدید 🗺️', 'ایونت هفتگی 🏆', 'رنک و آمار بازیکن 📊', 'چیز دیگه (توی ریپلای بگید)'],
  },
  {
    question: 'از پینگ و کیفیت سرور راضی هستید؟',
    options: ['آره، روانه ✅', 'بعضی وقتا لگ داره 😕', 'نه، مشکل داره ❌'],
  },
  {
    question: 'کدوم گیم‌مود رو بیشتر بازی می‌کنید؟',
    options: ['FFA ⚔️', 'NethPot / Pot 🧪', 'UHC 🍎', 'SMP 🌾'],
  },
  {
    question: 'لانچر چه قابلیتی داشته باشه که الان نداره؟',
    options: ['آپدیت خودکار مدها 🧩', 'اسکین و کازمتیک 👕', 'استتوس سرور توی لانچر 📶', 'چیز دیگه (ریپلای کنید)'],
  },
  {
    question: 'ساعت رویدادهای هفتگی چه زمانی مناسب‌تره؟',
    options: ['عصر (۵ تا ۸ شب) 🌆', 'شب (۸ تا ۱۱) 🌙', 'آخر هفته ظهرها ☀️'],
  },
];

function dayIndex() {
  return Math.floor(Date.now() / 86400000);
}

async function announceNewRelease() {
  const res = await fetch(
    'https://api.github.com/repos/pnbx/TCC-Launcher/releases/latest',
    { headers: { 'User-Agent': 'TCC-Telegram-Bot' } }
  );
  if (!res.ok) return;
  const release = await res.json().catch(() => null);
  if (!release?.tag_name || !release?.published_at) return;

  const age = Date.now() - new Date(release.published_at).getTime();
  if (age > 24 * 3600 * 1000) return; // not fresh; cron runs daily so each release is announced once

  await tg('sendMessage', {
    chat_id: telegramChat(),
    text: [
      `🚀 نسخه‌ی جدید لانچر TCC منتشر شد: ${release.tag_name}`,
      release.body ? `\n📝 ${release.body.slice(0, 400)}` : '',
      `\n⬇️ دانلود: ${DOWNLOAD_URL}`,
      '\nلانچر خودش موقع باز شدن آپدیت می‌شه — این لینک برای نصب اولیه‌ست.',
    ].join('\n'),
  });
}

async function sendDailyPoll() {
  const topic = TOPICS[dayIndex() % TOPICS.length];
  await tg('sendPoll', {
    chat_id: telegramChat(),
    question: topic.question,
    options: topic.options,
    is_anonymous: false,
  });
}

export default async function handler(req, res) {
  cors(res);

  // Vercel Cron sends x-vercel-cron; manual runs need ?key=CRON_SECRET.
  const authorized =
    req.headers['x-vercel-cron'] ||
    (process.env.CRON_SECRET && req.query?.key === process.env.CRON_SECRET);
  if (!authorized) return res.status(403).end();

  if (!telegramToken() || !telegramChat()) {
    return res.status(200).json({ ok: false, reason: 'TELEGRAM_BOT_TOKEN / TELEGRAM_CHAT_ID not configured' });
  }

  const results = {};
  try { await announceNewRelease(); results.release = 'ok'; } catch (e) { results.release = String(e); }
  try { await sendDailyPoll(); results.poll = 'ok'; } catch (e) { results.poll = String(e); }

  res.status(200).json({ ok: true, ...results });
}
