import test from "ava";

import { Mime } from "../index.js";

test("can parse", (t) => {
	t.deepEqual(Mime.parseStr("text/html;charset=utf-8"), Mime.TEXT_HTML());
	t.deepEqual(Mime.parseStr("application/json"), Mime.JSON());
	t.deepEqual(Mime.parseStr("image/png"), Mime.PNG());
	t.deepEqual(Mime.parseStr("image/jpg"), Mime.JPG());

	t.is(Mime.parseStr("invalidMime"), null);
});
