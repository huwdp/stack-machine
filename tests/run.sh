files=("test1" "test2" "test4" "test5" "test6" "test7" "test8" "test9")
for i in "${files[@]}"
do
	../target/debug/huw-stack-machine "$i.hsc" > "$i".output
  diff -q "$i.expected" "$i".output
done
rm -rf *.output
