export function timestampToWa(unixtime: number) {
	const date = new Date(unixtime * 1000);
	const yyyy = date.getFullYear();
	const mm = ("00" + (date.getMonth() + 1)).slice(-2);
	const dd = ("00" + date.getDate()).slice(-2);

	const hh = ("00" + date.getHours()).slice(-2);
	const min = ("00" + date.getMinutes()).slice(-2);
	const sec = ("00" + date.getSeconds()).slice(-2);

	return `${yyyy}${mm}${dd}${hh}${min}${sec}`;
}
