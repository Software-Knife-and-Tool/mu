;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; armv8 aarch64 instructions
;;;

;;;
;;; (prelude:load "/opt/mu/mu/arm-codegen/codegen.l" ())
;;; (arm:assemble-file (core:open :file :input "arm.s"))
;;;

;;;
;;; emit an imm12 field
;;;
(core:intern :arm "emit-imm12"
   (:lambda (range arg)
     (prelude:warn `(,range ,arg) "emit-imm12")
     ((:lambda (offset size value)
        (core:ash value (core:fx-sub offset (prelude:1- size))))
      (core:car range)
      (core:cdr range)
      (core:cdr arg))))

;;;
;;; emit a register field
;;;
(core:intern :arm "emit-reg"
   (:lambda (range arg)
     (prelude:warn `(,range ,arg) "emit-reg")
     ((:lambda (reg-map offset size)
        (prelude:warn (core:fx-sub offset (prelude:1- size)) "shift")
        (prelude:warn (prelude:assoc (core:keyword (core:sy-name (core:cdr arg))) reg-map) "reg value")
        (core:ash (core:cdr (prelude:assoc (core:keyword (core:sy-name (core:cdr arg))) reg-map))
                (core:fx-sub offset (prelude:1- size))))
      (core:cdr (prelude:assoc :regs arm:armv8-aarch64))
      (core:car range)
      (core:cdr range))))

;;;
;;; emit a const field
;;;
(core:intern :arm "emit-const"
     (:lambda (range arg)
       (prelude:warn `(,range ,arg) "emit-const")
       (prelude:warn (core:fx-sub (core:car range) (prelude:1- (core:cdr range))) "shift")
       (core:ash (core:cdr arg) (core:fx-sub (core:car range) (prelude:1- (core:cdr range))))))

;;;
;;; emitter dispatch
;;;
(core:intern :arm "field-emitter"
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
(core:intern :arm "emit-field"
   (:lambda (field-descriptor instruction-args)
     ((:lambda (tag range value)
        ((:lambda (inst-field emitter)
           (:if (prelude:null inst-field)
                (core:apply emitter `(,range ,(core:cons tag value)))
                (core:apply emitter `(,range ,inst-field))))
         (prelude:assoc tag instruction-args)
         (core:eval (core:cdr (prelude:assoc tag arm:field-emitter)))))
      (core:nth 0 field-descriptor)
      (core:nth 1 field-descriptor)
      (core:nth 2 field-descriptor))))

;;;
;;; loop over the opcode fields and construct the
;;; instruction binary value from the instruction arguments.
;;;
(core:intern :arm "emit-instruction"
   (:lambda (opcode-fields instruction-args)
     ((:lambda (config)
        (core:fix
         (:lambda (loop)
           (:if (prelude:numberp loop)
                loop
                ((:lambda (op field-list)
                   (:if (prelude:null field-list)
                        op
                        (core:cons
                         (core:logor op (arm:emit-field (core:car field-list) instruction-args))
                         (core:cdr field-list))))
                 (core:car loop)
                 (core:cdr loop))))
         (core:cons 0 opcode-fields)))
      (core:cdr (prelude:assoc :config arm:armv8-aarch64)))))     

;;;
;;; assemble an instruction from the instruction's opcode
;;; field list table entry and an alist of typed instruction
;;; arguments.
;;;
(core:intern :arm "assemble-instruction"
   (:lambda (instruction opcode-format opcode-fields)
     ((:lambda (format-length)
        (arm:emit-instruction
         opcode-fields
         (core:cdr
          (core:fix
           (:lambda (loop)
             ((:lambda (nth acc)
                (:if (core:fx-lt nth format-length)
                     (core:cons (prelude:1+ nth)
                              (core:append
                               acc
                               `(,(core:cons (core:nth nth opcode-format)
                                           (core:nth (prelude:1+ nth) instruction)))))
                     loop))
              (core:car loop)
              (core:cdr loop)))
           '(0 . ())))))
      (core:length opcode-format))))

;;;
;;; assemble an instruction to binary
;;;
(core:intern :arm "armv8-assemble"
   (:lambda (instruction)
     ((:lambda (opcodes)
        ((:lambda (opcode-descriptor)
           ((:lambda (opcode-format opcode-fields)
              (:if (prelude:null opcode-format)
                   ()
                   (:if (core:eq (core:length opcode-format) (core:length instruction))
                        (arm:assemble-instruction instruction opcode-format opcode-fields)
                        (prelude:format () "~A argument arity: expected ~A, got ~A"
                                        `(,(core:car instruction)
                                           ,(core:length opcode-format)
                                           ,(core:length instruction))))))
            (core:cdr (prelude:assoc :format opcode-descriptor))
            (core:cdr (prelude:assoc :fields opcode-descriptor))))
         (core:cdr (prelude:assoc (core:car instruction) opcodes))))
      (core:cdr (prelude:assoc :opcodes arm:armv8-aarch64)))))

(core:intern :arm "armv8-aarch64"
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
