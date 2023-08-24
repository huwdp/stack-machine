files=("test1" "test2" "test4" "test5" "test6" "test7")
for i in "${files[@]}"
do
	../main "$i.hsc" > "$i".output
  diff -q "$i.expected" "$i".output
done
rm -rf *.output
