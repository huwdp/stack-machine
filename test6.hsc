push 20
label Label1
push 20
je Label2
j end

label Label2
push 0
jn Label3
j end

label Label3
push 30
jg Label4
j end

label Label4
push 20
jge Label5
j end

label Label5
push 10
jl Label6
j end

label Label6
push 20
jle Label7
j end

label Label7
push 30
jg Label8
j end

label Label8
push 20
j end

label end
push 69
print
