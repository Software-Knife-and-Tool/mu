;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; armv8 aarch64 instructions
;;;

;;;
;;; (prelude:load "/opt/mu/mu/arm-codegen/codegen.l" ())
;;; (arm:assemble-file (libcore:open :file :input "arm.s"))
;;;

;;;
;;; emit an imm12 field
;;;
(libcore:intern :arm "emit-imm12"
   (:lambda (range arg)
     (prelude:warn `(,range ,arg) "emit-imm12")
     ((:lambda (offset size value)
        (libcore:ash value (libcore:fx-sub offset (prelude:1- size))))
      (libcore:car range)
      (libcore:cdr range)
      (libcore:cdr arg))))

;;;
;;; emit a register field
;;;
(libcore:intern :arm "emit-reg"
   (:lambda (range arg)
     (prelude:warn `(,range ,arg) "emit-reg")
     ((:lambda (reg-map offset size)
        (prelude:warn (libcore:fx-sub offset (prelude:1- size)) "shift")
        (prelude:warn (prelude:assoc (libcore:keyword (libcore:sy-name (libcore:cdr arg))) reg-map) "reg value")
        (libcore:ash (libcore:cdr (prelude:assoc (libcore:keyword (libcore:sy-name (libcore:cdr arg))) reg-map))
                (libcore:fx-sub offset (prelude:1- size))))
      (libcore:cdr (prelude:assoc :regs arm:armv8-aarch64))
      (libcore:car range)
      (libcore:cdr range))))

;;;
;;; emit a const field
;;;
(libcore:intern :arm "emit-const"
     (:lambda (range arg)
       (prelude:warn `(,range ,arg) "emit-const")
       (prelude:warn (libcore:fx-sub (libcore:car range) (prelude:1- (libcore:cdr range))) "shift")
       (libcore:ash (libcore:cdr arg) (libcore:fx-sub (libcore:car range) (prelude:1- (libcore:cdr range))))))

;;;
;;; emitter dispatch
;;;
(libcore:intern :arm "field-emitter"
    `((:sf . arm:emit-const)
      (:op . arm:emit-const)
      (:sh . arm:emit-const)
      (:imm12 . arm:emit-imm12)
      (:rn . arm:emit-reg)
      (:rd . arm:emit-reg)))

;;;
;;; decode a field type and call the appropriate emitter.
;;;
;;; return the emitted binary value
;;;
(libcore:intern :arm "emit-field"
   (:lambda (field-descriptor instruction-args)
     ((:lambda (tag range value)
        ((:lambda (inst-field emitter)
           (:if (prelude:null inst-field)
                (libcore:apply emitter `(,range ,(libcore:cons tag value)))
                (libcore:apply emitter `(,range ,inst-field))))
         (prelude:assoc tag instruction-args)
         (libcore:eval (libcore:cdr (prelude:assoc tag arm:field-emitter)))))
      (libcore:nth 0 field-descriptor)
      (libcore:nth 1 field-descriptor)
      (libcore:nth 2 field-descriptor))))

;;;
;;; loop over the opcode fields and construct the
;;; instruction binary value from the instruction arguments.
;;;
(libcore:intern :arm "emit-instruction"
   (:lambda (opcode-fields instruction-args)
     ((:lambda (config)
        (libcore:fix
         (:lambda (loop)
           (:if (prelude:numberp loop)
                loop
                ((:lambda (op field-list)
                   (:if (prelude:null field-list)
                        op
                        (libcore:cons
                         (libcore:logor op (arm:emit-field (libcore:car field-list) instruction-args))
                         (libcore:cdr field-list))))
                 (libcore:car loop)
                 (libcore:cdr loop))))
         (libcore:cons 0 opcode-fields)))
      (libcore:cdr (prelude:assoc :config arm:armv8-aarch64)))))     

;;;
;;; assemble an instruction from the instruction's opcode
;;; field list table entry and an alist of typed instruction
;;; arguments.
;;;
(libcore:intern :arm "assemble-instruction"
   (:lambda (instruction opcode-format opcode-fields)
     ((:lambda (format-length)
        (arm:emit-instruction
         opcode-fields
         (libcore:cdr
          (libcore:fix
           (:lambda (loop)
             ((:lambda (nth acc)
                (:if (libcore:fx-lt nth format-length)
                     (libcore:cons (prelude:1+ nth)
                              (libcore:append
                               acc
                               `(,(libcore:cons (libcore:nth nth opcode-format)
                                           (libcore:nth (prelude:1+ nth) instruction)))))
                     loop))
              (libcore:car loop)
              (libcore:cdr loop)))
           '(0 . ())))))
      (libcore:length opcode-format))))

;;;
;;; assemble an instruction to binary
;;;
(libcore:intern :arm "armv8-assemble"
   (:lambda (instruction)
     ((:lambda (opcodes)
        ((:lambda (opcode-descriptor)
           ((:lambda (opcode-format opcode-fields)
              (:if (prelude:null opcode-format)
                   ()
                   (:if (libcore:eq (libcore:length opcode-format) (libcore:length instruction))
                        (arm:assemble-instruction instruction opcode-format opcode-fields)
                        (prelude:format () "~A argument arity: expected ~A, got ~A"
                                        `(,(libcore:car instruction)
                                           ,(libcore:length opcode-format)
                                           ,(libcore:length instruction))))))
            (libcore:cdr (prelude:assoc :format opcode-descriptor))
            (libcore:cdr (prelude:assoc :fields opcode-descriptor))))
         (libcore:cdr (prelude:assoc (libcore:car instruction) opcodes))))
      (libcore:cdr (prelude:assoc :opcodes arm:armv8-aarch64)))))

(libcore:intern :arm "armv8-aarch64"
   '((:config . ((:arch :aarch64)))
     (:opcodes .
      ((:add.im .
         ((:format . (:rn :rd :imm12 :imm1))
          (:fields . ((:sf (31 . 1) 1)
                      (:op (28 . 6) #x22)
                      (:sh (22 . 1) 0)
                      (:imm12 (21 . 12) ())
                      (:rn (9 . 5) ())
                      (:rd (4 . 5) ())))))))
     (:regs .
      ((:x0 . 0)
       (:x1 . 1)
       (:x2 . 2)
       (:x3 . 3)
       (:x4 . 4)
       (:x5 . 5)
       (:x6 . 6)
       (:x7 . 7)
       (:x8 . 8)
       (:x9 . 9)
       (:x10 . 10)
       (:x11 . 11)
       (:x12 . 12)
       (:x13 . 13)
       (:x14 . 14)
       (:x15 . 15)
       (:x16 . 16)
       (:x17 . 17)
       (:x18 . 18)
       (:x19 . 19)
       (:x20 . 20)
       (:x21 . 21)
       (:x22 . 22)
       (:x23 . 23)
       (:x24 . 24)
       (:x25 . 25)
       (:x26 . 26)
       (:x27 . 27)
       (:x28 . 28)
       (:x29 . 29)
       (:x30 . 30)
       (:x31 . 31)))))
