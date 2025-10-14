;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

#|
CONCATENATE
COPY-SEQ [x]
COUNT, COUNT-IF [x]
ELT [x]
FIND, FIND-IF [x]
LENGTH [x]
MAKE-SEQUENCE
MAP
MISMATCH
POSITION, POSITION-IF [x]
REDUCE
REMOVE, REMOVE-IF [x]
REMOVE-DUPLICATES
REVERSE [x]
SEARCH
SUBSEQ
SUBSTITUTE, SUBSTITUTE-IF [x]
|#

;;;
;;; sequences
;;;
#|
(mu:intern mu:*null/* "map"
   (core:compile
    '(%lambda (result-type function &rest sequences)
       ((:lambda (min-length lists function)
          (mu:cdr (mu:cdr
                   (mu:fix
                    (:lambda (loop)
                      ((:lambda (nth args)
                         (:if (mu:less-than nth min-length)
                              (:if (mu:eq :func (mu:type-of function))
                                   (mu:cons (mu:add 1 nth)
                                            (core:%append
                                             `(,args
                                               ,(mu:cons (mu:apply function (common:%nths nth lists))
                                                         ()))))                             
                                   (mu:cons (mu:add 1 nth)
                                            (core:%append
                                             `(,args
                                               ,(mu:cons (core:apply function (common:%nths nth lists))
                                                         ())))))
                              loop))
                       (mu:car loop)
                       (mu:cdr loop)))
                    '(0 ())))))
        (core:apply common:min (core:%mapcar length sequences))
        lists
        function))))

(mu:intern common "%make-sequence"
   (:lambda (result-type size init)
      ((:lambda (list)
         (:if (mu:eq :vector result-type)
              (mu:make-vector :t list)
              (:if (core:null :list)
                   list
                   (core:error "%make-sequence: unimplemented result type, ~A~%" `(,result-type)))))
       (mu:cdr
        (mu:fix
         (:lambda (loop)
           ((:lambda (nth list)
              (:if (mu:eq 0 nth)
                   loop
                   (mu:cons (mu:sub nth 1) (mu:cons init list))))
            (mu:car loop)
            (mu:cdr loop))
           (mu:cons size ())))))))
|#

;;;
;;; generic primitives
;;;
(%defgeneric
 length
 (:vector . mu:vector-length)
 (:cons   . mu:length)
 (:null   . (:lambda (seq) 0)))

(%defgeneric
 elt
 (:vector . mu:svref)
 (:cons   . (:lambda (seq nth)
              (:if (mu:less-than nth (mu:length seq))
                   (mu:nth nth seq)
                   (core:error "elt: invalid index, ~A~%" `(,nth))))))

(%defgeneric
 copy-seq
 (:vector . (:lambda (seq)
              (mu:make-vector
               (mu:vector-type seq)
               (mu:car
                (mu:fix
                 (:lambda (loop)
                   ((:lambda (copy nth)
                      (:if (mu:less-than nth 0)
                           loop
                           (mu:cons (mu:cons (mu:svref seq nth) copy) nth)))
                    (mu:car loop)
                    (mu:sub (mu:cdr loop) 1)))
                 (mu:cons () (mu:vector-length seq)))))))
 (:cons   . (:lambda (seq) (core:%mapcar core:identity seq)))
 (:null   . (:lambda (seq) ())))

(%defgeneric
 %find
 (:vector . (:lambda (seq item)
              (mu:cdr
               (mu:fix
                (:lambda (loop)
                  ((:lambda (done nth)
                     (:if (core:null done)
                          (:if (mu:eq nth 0)
                               (mu:cons :t ())
                               (:if (mu:eq item (mu:svref seq (mu:sub nth 1)))
                                    (mu:cons :t item)
                                    (mu:cons () (mu:sub nth 1))))
                          loop))
                   (mu:car loop)
                   (mu:cdr loop)))
                (mu:cons () (mu:vector-length seq))))))
 (:cons   . (:lambda (seq item) (core:%find-if (:lambda (elt) (mu:eq elt item)) seq)))
 (:null   . (:lambda (seq item) ())))

(%defgeneric
 %find-if
 (:vector . (:lambda (seq fn)
              (mu:cdr
               (mu:fix
                (:lambda (loop)
                  ((:lambda (done nth)
                     (:if (core:null done)
                          (:if (mu:eq nth 0)
                               (mu:cons :t ())
                               ((:lambda (item)
                                  (:if (core:apply fn `(,item))
                                       (mu:cons :t item)
                                       (mu:cons () (mu:sub nth 1))))
                                (mu:svref seq (mu:sub nth 1))))
                          loop))
                   (mu:car loop)
                   (mu:cdr loop)))
                (mu:cons () (mu:vector-length seq))))))
 (:cons   . (:lambda (seq fn) (core:%find-if fn seq)))
 (:null   . (:lambda (seq fn) ())))

(%defgeneric
 %count
 (:vector . (:lambda (seq item)
              (mu:car
               (mu:fix
                (:lambda (loop)
                  ((:lambda (count nth)
                     (:if (mu:less-than nth 0)
                          loop
                          (:if (mu:eq item (mu:svref seq nth))
                               (mu:cons (mu:add 1 count) nth)
                               (mu:cons count nth))))
                  (mu:car loop)
                  (mu:sub (mu:cdr loop) 1)))
                (mu:cons 0 (mu:vector-length seq))))))
 (:cons   . (:lambda (seq item)
              (mu:car
               (mu:fix
                (:lambda (loop)
                  ((:lambda (count cons)
                     (:if (core:null cons)
                          loop
                          (:if (mu:eq item (mu:car cons))
                               (mu:cons (mu:add 1 count) (mu:cdr cons))
                               (mu:cons count (mu:cdr cons)))))
                  (mu:car loop)
                  (mu:cdr loop)))
                (mu:cons 0 seq)))))
 (:null   . (:lambda (seq item) 0)))

(%defgeneric
 %count-if
 (:vector . (:lambda (seq fn)
              (mu:car
               (mu:fix
                (:lambda (loop)
                  ((:lambda (count nth)
                     (:if (mu:less-than nth 0)
                          loop
                          (:if (core:apply fn `(,(mu:svref seq nth)))
                                  (mu:cons (mu:add 1 count) nth)
                                  (mu:cons count nth))))
                  (mu:car loop)
                  (mu:sub (mu:cdr loop) 1)))
               (mu:cons 0 (mu:vector-length seq))))))
 (:cons . (:lambda (seq fn)
              (mu:car
               (mu:fix
                (:lambda (loop)
                  ((:lambda (count cons)
                     (:if (core:null cons)
                          loop
                          (:if (core:apply fn `(,(mu:car cons)))
                               (mu:cons (mu:add 1 count) (mu:cdr cons))
                               (mu:cons count (mu:cdr cons)))))
                  (mu:car loop)
                  (mu:cdr loop)))
                (mu:cons 0 seq)))))
 (:null   . (:lambda (seq fn) 0)))

(%defgeneric
 %position
 (:vector . (:lambda (seq item)
              ((:lambda (length)
                 (mu:fix
                  (:lambda (nth)
                    (:if (core:null nth)
                         ()
                         (:if (mu:eq nth length)
                           ()
                           (:if (mu:eq (mu:svref seq nth) item)
                                nth
                                (mu:add 1 nth)))))
                  0))
               (mu:vector-length seq))))
 (:cons    . (:lambda (seq item)
              ((:lambda (length)
                 (mu:fix
                  (:lambda (nth)
                    (:if (core:null nth)
                         ()
                         (:if (mu:eq nth length)
                              ()
                              (:if (mu:eq (mu:nth nth seq) item)
                                   nth
                                   (mu:add 1 nth)))))
                  0))
               (mu:length seq))))
 (:null   . (:lambda (seq item) ())))

(%defgeneric
 %position-if
 (:vector . (:lambda (seq fn)
              ((:lambda (length)
                 (mu:fix
                  (:lambda (nth)
                    (:if (core:null nth)
                         ()
                         (:if (mu:eq nth length)
                           ()
                           (:if (core:apply fn `(,(mu:svref seq nth)))
                                nth
                                (mu:add 1 nth)))))
                  0))
               (mu:vector-length seq))))
 (:cons   . (:lambda (seq fn)
              ((:lambda (length)
                 (mu:fix
                  (:lambda (nth)
                    (:if (core:null nth)
                         ()
                         (:if (mu:eq nth length)
                           ()
                           (:if (core:apply fn `(,(mu:nth nth seq)))
                                nth
                                (mu:add 1 nth)))))
                  0))
               (mu:length seq))))
 (:null   . (:lambda (seq item) ())))

(%defgeneric
 %remove
 (:vector . (:lambda (seq item)
              ((:lambda (length)
                 (mu:make-vector
                  (mu:vector-type seq)
                  (mu:cdr
                   (mu:fix
                    (:lambda (loop)
                      ((:lambda (nth list)
                         (:if (mu:eq nth length)
                              loop
                              ((:lambda (elt)
                                 (:if (mu:eq item elt)
                                      (mu:cons (mu:add 1 nth) list)
                                      (mu:cons (mu:add 1 nth) (mu:cons elt list))))
                               (mu:svref seq nth))))
                       (mu:car loop)
                       (mu:cdr loop)))
                    (mu:cons 0 ())))))
                 (mu:vector-length seq))))
 (:cons   . (:lambda (seq item)
              ((:lambda (length)
                 (mu:cdr
                  (mu:fix
                   (:lambda (loop)
                     ((:lambda (nth list)
                        (:if (mu:eq nth (mu:length seq))
                             loop
                             ((:lambda (elt)
                                (:if (mu:eq item elt)
                                     (mu:cons (mu:add 1 nth) list)
                                     (mu:cons (mu:add 1 nth) (mu:cons elt list))))
                              (mu:nth nth seq))))
                      (mu:car loop)
                      (mu:cdr loop)))
                   (mu:cons 0 ()))))
               (mu:length seq))))
 (:null   . (:lambda (seq item) ())))

(%defgeneric
 %remove-if
 (:vector . (:lambda (seq predicate)
              ((:lambda (length)
                 (mu:make-vector
                  (mu:vector-type seq)
                  (mu:cdr
                   (mu:fix
                    (:lambda (loop)
                      ((:lambda (nth list)
                         (:if (mu:eq nth length)
                           loop
                           ((:lambda (elt)
                              (:if (core:apply predicate `(,elt))
                                   (mu:cons (mu:add 1 nth) list)
                                   (mu:cons (mu:add 1 nth) (mu:cons elt list))))
                            (mu:svref seq nth))))
                    (mu:car loop)
                    (mu:cdr loop)))
                    (mu:cons 0 ())))))
               (mu:vector-length seq))))
 (:cons   . (:lambda (seq predicate)
              ((:lambda (length)
                 (mu:cdr
                  (mu:fix
                   (:lambda (loop)
                     ((:lambda (nth list)
                        (:if (mu:eq nth (mu:length seq))
                             loop
                             ((:lambda (elt)
                                (:if (core:apply predicate `(,elt))
                                     (mu:cons (mu:add 1 nth) list)
                                     (mu:cons (mu:add 1 nth) (mu:cons elt list))))
                              (mu:nth nth seq))))
                      (mu:car loop)
                      (mu:cdr loop)))
                   (mu:cons 0 ()))))
               (mu:length seq))))
 (:null   . (:lambda (seq item) ())))

(%defgeneric
 %substitute
 (:vector . (:lambda (seq new old)
              (mu:make-vector
               (mu:vector-type seq)
               (mu:cdr
                (mu:fix
                 (:lambda (loop)
                   ((:lambda (nth list)
                      (:if (mu:less-than nth 0)
                           loop
                           ((:lambda (elt)
                              (:if (mu:eq old elt)
                                   (mu:cons nth (mu:cons new list))
                                   (mu:cons nth (mu:cons elt list))))
                            (mu:svref seq nth))))
                    (mu:sub (mu:car loop) 1)
                    (mu:cdr loop)))
                 (mu:cons (mu:vector-length seq) ()))))))
 (:cons   . (:lambda (seq new old)
              (mu:cdr
               (mu:fix
                (:lambda (loop)
                  ((:lambda (nth list)
                     (:if (mu:less-than nth 0)
                          loop
                          ((:lambda (elt)
                             (:if (mu:eq old elt)
                                  (mu:cons nth (mu:cons new list))
                                  (mu:cons nth (mu:cons elt list))))
                           (mu:nth nth seq))))
                   (mu:sub (mu:car loop) 1)
                   (mu:cdr loop)))
                   (mu:cons (mu:length seq) ())))))
 (:null   . (:lambda (seq new old) ())))

(%defgeneric
 %substitute-if
 (:vector . (:lambda (seq new predicate)
              (mu:make-vector
               (mu:vector-type seq)
               (mu:cdr
                (mu:fix
                 (:lambda (loop)
                   ((:lambda (nth list)
                      (:if (mu:less-than nth 0)
                           loop
                           ((:lambda (elt)
                              (:if (core:apply predicate `(,elt))
                                   (mu:cons nth (mu:cons new list))
                                   (mu:cons nth (mu:cons elt list))))
                            (mu:svref seq nth))))
                    (mu:sub (mu:car loop) 1)
                    (mu:cdr loop)))
                 (mu:cons (mu:vector-length seq) ()))))))
 (:cons   . (:lambda (seq new predicate)
                 (mu:cdr
                  (mu:fix
                   (:lambda (loop)
                     ((:lambda (nth list)
                        (:if (mu:less-than nth 0)
                             loop
                             ((:lambda (elt)
                                (:if (core:apply predicate `(,elt))
                                     (mu:cons nth (mu:cons new list))
                                     (mu:cons nth (mu:cons elt list))))
                              (mu:nth nth seq))))
                      (mu:sub (mu:car loop) 1)
                      (mu:cdr loop)))
                   (mu:cons (mu:length seq) ())))))
 (:null   . (:lambda (seq new old) ())))

(%defgeneric
 reverse
 (:vector . (:lambda (seq)
              ((:lambda (length)
                 (mu:make-vector
                  (mu:vector-type seq)
                  (mu:cdr
                   (mu:fix
                    (:lambda (loop)
                      ((:lambda (nth list)
                         (:if (mu:eq nth length)
                              loop
                              ((:lambda (elt)
                                 (mu:cons (mu:add 1 nth) (mu:cons elt list)))
                               (mu:svref seq nth))))
                     (mu:car loop)
                     (mu:cdr loop)))
                    (mu:cons 0 ())))))
               (mu:vector-length seq))))
 (:cons   . core:reverse)
 (:null   . (:lambda (seq) ())))
