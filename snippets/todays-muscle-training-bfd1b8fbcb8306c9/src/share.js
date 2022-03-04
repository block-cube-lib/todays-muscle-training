export async function share(title, text) {
  if (!window.navigator.share) {
    window.alert("お使いのブラウザではシェアできません");
    console.log(text + "\n" + location.href);
    return;
  }

  try {
    await window.navigator.share({
      title: title,
      text: text,
      url: location.href,
    });
  } catch (e) {
    console.log(e.message);
  }
}