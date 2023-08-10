push 20
lbl Label1
push 20
je Label2
j end

lbl Label2
push 0
jn Label3
j end

lbl Label3
push 30
jg Label4
j end

lbl Label4
push 20
jge Label5
j end

lbl Label5
push 10
jl Label6
j end

lbl Label6
push 20
jle Label7
j end

lbl Label7
push 30
jg Label8
j end

lbl Label8
push 20
j end

lbl end
push 69
prt
