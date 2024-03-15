;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; armv8 aarch64 instructions
;;;

;;;
;;; (prelude:load "/opt/mu/mu/arm-codegen/codegen.l" ())
;;; (arm:assemble-file (mu:open :file :input "arm.s"))
;;;

;;;
;;; emit an imm12 field
;;;
(mu:intern :arm "emit-imm12"
   (:lambda (range arg)
     (prelude:warn `(,range ,arg) "emit-imm12")
     ((:lambda (offset size value)
        (mu:ash value (mu:fx-sub offset (prelude:1- size))))
      (mu:car range)
      (mu:cdr range)
      (mu:cdr arg))))

;;;
;;; emit a register field
;;;
(mu:intern :arm "emit-reg"
   (:lambda (range arg)
     (prelude:warn `(,range ,arg) "emit-reg")
     ((:lambda (reg-map offset size)
        (prelude:warn (mu:fx-sub offset (prelude:1- size)) "shift")
        (prelude:warn (prelude:assoc (mu:keyword (mu:sy-name (mu:cdr arg))) reg-map) "reg value")
        (mu:ash (mu:cdr (prelude:assoc (mu:keyword (mu:sy-name (mu:cdr arg))) reg-map))
                (mu:fx-sub offset (prelude:1- size))))
      (mu:cdr (prelude:assoc :regs arm:armv8-aarch64))
      (mu:car range)
      (mu:cdr range))))

;;;
;;; emit a const field
;;;
(mu:intern :arm "emit-const"
     (:lambda (range arg)
       (prelude:warn `(,range ,arg) "emit-const")
       (prelude:warn (mu:fx-sub (mu:car range) (prelude:1- (mu:cdr range))) "shift")
       (mu:ash (mu:cdr arg) (mu:fx-sub (mu:car range) (prelude:1- (mu:cdr range))))))

;;;
;;; emitter dispatch
;;;
(mu:intern :arm "field-emitter"
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
(mu:intern :arm "emit-field"
   (:lambda (field-descriptor instruction-args)
     ((:lambda (tag range value)
        ((:lambda (inst-field emitter)
           (:if (prelude:null inst-field)
                (mu:apply emitter `(,range ,(mu:cons tag value)))
                (mu:apply emitter `(,range ,inst-field))))
         (prelude:assoc tag instruction-args)
         (mu:eval (mu:cdr (prelude:assoc tag arm:field-emitter)))))
      (mu:nth 0 field-descriptor)
      (mu:nth 1 field-descriptor)
      (mu:nth 2 field-descriptor))))

;;;
;;; loop over the opcode fields and construct the
;;; instruction binary value from the instruction arguments.
;;;
(mu:intern :arm "emit-instruction"
   (:lambda (opcode-fields instruction-args)
     ((:lambda (config)
        (mu:fix
         (:lambda (loop)
           (:if (prelude:numberp loop)
                loop
                ((:lambda (op field-list)
                   (:if (prelude:null field-list)
                        op
                        (mu:cons
                         (mu:logor op (arm:emit-field (mu:car field-list) instruction-args))
                         (mu:cdr field-list))))
                 (mu:car loop)
                 (mu:cdr loop))))
         (mu:cons 0 opcode-fields)))
      (mu:cdr (prelude:assoc :config arm:armv8-aarch64)))))     

;;;
;;; assemble an instruction from the instruction's opcode
;;; field list table entry and an alist of typed instruction
;;; arguments.
;;;
(mu:intern :arm "assemble-instruction"
   (:lambda (instruction opcode-format opcode-fields)
     ((:lambda (format-length)
        (arm:emit-instruction
         opcode-fields
         (mu:cdr
          (mu:fix
           (:lambda (loop)
             ((:lambda (nth acc)
                (:if (mu:fx-lt nth format-length)
                     (mu:cons (prelude:1+ nth)
                              (mu:append
                               acc
                               `(,(mu:cons (mu:nth nth opcode-format)
                                           (mu:nth (prelude:1+ nth) instruction)))))
                     loop))
              (mu:car loop)
              (mu:cdr loop)))
           '(0 . ())))))
      (mu:length opcode-format))))

;;;
;;; assemble an instruction to binary
;;;
(mu:intern :arm "armv8-assemble"
   (:lambda (instruction)
     ((:lambda (opcodes)
        ((:lambda (opcode-descriptor)
           ((:lambda (opcode-format opcode-fields)
              (:if (prelude:null opcode-format)
                   ()
                   (:if (mu:eq (mu:length opcode-format) (mu:length instruction))
                        (arm:assemble-instruction instruction opcode-format opcode-fields)
                        (prelude:format () "~A argument arity: expected ~A, got ~A"
                                        `(,(mu:car instruction)
                                           ,(mu:length opcode-format)
                                           ,(mu:length instruction))))))
            (mu:cdr (prelude:assoc :format opcode-descriptor))
            (mu:cdr (prelude:assoc :fields opcode-descriptor))))
         (mu:cdr (prelude:assoc (mu:car instruction) opcodes))))
      (mu:cdr (prelude:assoc :opcodes arm:armv8-aarch64)))))

(mu:intern :arm "armv8-aarch64"
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
