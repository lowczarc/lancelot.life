function html_like_to_rust_struct {
	cat $1 | sed 's/^[ \t]*//;s/[ \t]*$//' | tr -d "\n" | sed \
		-e 's/"/\\"/g' \
		-e 's/{\([a-zA-Z.]\+\) |\([a-zA-Z]\+\)| \[/"), Value(Array("\1", "\2", \&[Litteral("/g' \
		-e 's/\]}/")\])), Litteral("/g' \
		-e 's/{\([a-zA-Z.]\+\)}/"), Value(Content("\1")), Litteral("/g' \
		-e 's/}/)),/g' \
		-e 's/^/\&[ Litteral("/' \
		-e 's/$/") ]/' > $1.in
}

for f in views/*.html
do
	html_like_to_rust_struct $f
done
