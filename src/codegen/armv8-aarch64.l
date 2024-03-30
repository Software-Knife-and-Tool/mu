;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; armv8 aarch64 instructions
;;;

;;;
;;; (prelude:load "/opt/mu/mu/arm-codegen/codegen.l" ())
;;; (arm:assemble-file (lib:open :file :input "arm.s"))
;;;

;;;
;;; emit an imm12 field
;;;
(lib:intern :arm "emit-imm12"
   (:lambda (range arg)
     (prelude:warn `(,range ,arg) "emit-imm12")
     ((:lambda (offset size value)
        (lib:ash value (lib:fx-sub offset (prelude:1- size))))
      (lib:car range)
      (lib:cdr range)
      (lib:cdr arg))))

;;;
;;; emit a register field
;;;
(lib:intern :arm "emit-reg"
   (:lambda (range arg)
     (prelude:warn `(,range ,arg) "emit-reg")
     ((:lambda (reg-map offset size)
        (prelude:warn (lib:fx-sub offset (prelude:1- size)) "shift")
        (prelude:warn (prelude:assoc (lib:keyword (lib:sy-name (lib:cdr arg))) reg-map) "reg value")
        (lib:ash (lib:cdr (prelude:assoc (lib:keyword (lib:sy-name (lib:cdr arg))) reg-map))
                (lib:fx-sub offset (prelude:1- size))))
      (lib:cdr (prelude:assoc :regs arm:armv8-aarch64))
      (lib:car range)
      (lib:cdr range))))

;;;
;;; emit a const field
;;;
(lib:intern :arm "emit-const"
     (:lambda (range arg)
       (prelude:warn `(,range ,arg) "emit-const")
       (prelude:warn (lib:fx-sub (lib:car range) (prelude:1- (lib:cdr range))) "shift")
       (lib:ash (lib:cdr arg) (lib:fx-sub (lib:car range) (prelude:1- (lib:cdr range))))))

;;;
;;; emitter dispatch
;;;
(lib:intern :arm "field-emitter"
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
(lib:intern :arm "emit-field"
   (:lambda (field-descriptor instruction-args)
     ((:lambda (tag range value)
        ((:lambda (inst-field emitter)
           (:if (prelude:null inst-field)
                (lib:apply emitter `(,range ,(lib:cons tag value)))
                (lib:apply emitter `(,range ,inst-field))))
         (prelude:assoc tag instruction-args)
         (lib:eval (lib:cdr (prelude:assoc tag arm:field-emitter)))))
      (lib:nth 0 field-descriptor)
      (lib:nth 1 field-descriptor)
      (lib:nth 2 field-descriptor))))

;;;
;;; loop over the opcode fields and construct the
;;; instruction binary value from the instruction arguments.
;;;
(lib:intern :arm "emit-instruction"
   (:lambda (opcode-fields instruction-args)
     ((:lambda (config)
        (lib:fix
         (:lambda (loop)
           (:if (prelude:numberp loop)
                loop
                ((:lambda (op field-list)
                   (:if (prelude:null field-list)
                        op
                        (lib:cons
                         (lib:logor op (arm:emit-field (lib:car field-list) instruction-args))
                         (lib:cdr field-list))))
                 (lib:car loop)
                 (lib:cdr loop))))
         (lib:cons 0 opcode-fields)))
      (lib:cdr (prelude:assoc :config arm:armv8-aarch64)))))     

;;;
;;; assemble an instruction from the instruction's opcode
;;; field list table entry and an alist of typed instruction
;;; arguments.
;;;
(lib:intern :arm "assemble-instruction"
   (:lambda (instruction opcode-format opcode-fields)
     ((:lambda (format-length)
        (arm:emit-instruction
         opcode-fields
         (lib:cdr
          (lib:fix
           (:lambda (loop)
             ((:lambda (nth acc)
                (:if (lib:fx-lt nth format-length)
                     (lib:cons (prelude:1+ nth)
                              (lib:append
                               acc
                               `(,(lib:cons (lib:nth nth opcode-format)
                                           (lib:nth (prelude:1+ nth) instruction)))))
                     loop))
              (lib:car loop)
              (lib:cdr loop)))
           '(0 . ())))))
      (lib:length opcode-format))))

;;;
;;; assemble an instruction to binary
;;;
(lib:intern :arm "armv8-assemble"
   (:lambda (instruction)
     ((:lambda (opcodes)
        ((:lambda (opcode-descriptor)
           ((:lambda (opcode-format opcode-fields)
              (:if (prelude:null opcode-format)
                   ()
                   (:if (lib:eq (lib:length opcode-format) (lib:length instruction))
                        (arm:assemble-instruction instruction opcode-format opcode-fields)
                        (prelude:format () "~A argument arity: expected ~A, got ~A"
                                        `(,(lib:car instruction)
                                           ,(lib:length opcode-format)
                                           ,(lib:length instruction))))))
            (lib:cdr (prelude:assoc :format opcode-descriptor))
            (lib:cdr (prelude:assoc :fields opcode-descriptor))))
         (lib:cdr (prelude:assoc (lib:car instruction) opcodes))))
      (lib:cdr (prelude:assoc :opcodes arm:armv8-aarch64)))))

(lib:intern :arm "armv8-aarch64"
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
