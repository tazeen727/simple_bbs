const minHeight = 60;
const maxHeight = 300;

/*------ テキストエリアの高さを内容に従って調節する -----*/
function adjustCommentHeight() {
    const target = document.getElementById("comment");
    target.style.height = "auto";

    let ht = target.scrollHeight + 5;
    if (ht < minHeight) {
        ht = minHeight;
    }
    if (ht > maxHeight) {
        ht = maxHeight;
        target.style.overflowY = "scroll";
    } else {
        target.style.overflowY = "hidden"
    }
    const heightPx = ht + "px";
    target.style.height = heightPx;
}

/* 画面描画時に一度呼び出す */
adjustCommentHeight();

/* コメントを編集するたびに高さを調節する */
document.getElementById("comment").addEventListener("keyup", adjustCommentHeight);


/*-------- 投稿一覧ページで一番上にスクロールするボタン -------*/
document.getElementById("to-the-top").addEventListener("click", (e) => {
    e.preventDefault();
    scrollTo({
        top: 0,
        behavior: "smooth"
    });
});

/* 現在のスクロール位置が一定(1000px)より下のときだけ表示する */
const threshold = 1000;
function controlVisibility() {
    let posY = window.scrollY;
    let btn = document.getElementById("to-the-top");
    if (posY < threshold) {
        btn.style.display = "none";
    } else {
        btn.style.display = "block";
    }

    // 200ms毎に自分を呼び出す
    setTimeout(controlVisibility, 500);
}

// 画面描画が終わったら開始
window.addEventListener("load", controlVisibility);
