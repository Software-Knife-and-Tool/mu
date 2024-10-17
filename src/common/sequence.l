;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

#|
System Class SEQUENCE 
Accessor SUBSEQ
Function CONCATENATE
Function FILL
Function MAKE-SEQUENCE
Function MAP
Function MAP-INTO
Function MERGE
Function MISMATCH
Function REDUCE
Function REMOVE-DUPLICATES
Function REPLACE
Function SEARCH
Function SORT, STABLE-SORT
Function SUBSTITUTE, SUBSTITUTE-IF, SUBSTITUTE-IF-NOT
|#

;;;
;;; sequences
;;;
(mu:intern common "sequencep"
   (:lambda (t)
     (:if (core:listp t)
          :t
          (core:vectorp t))))

(mu:intern common "length"
   (:lambda (seq)
      (:if (common:sequencep seq)
           (:if (core:listp seq)
                (mu:length seq)
                (mu:vector-len seq))
           (core:raise seq 'common:length "not a sequence"))))

(mu:intern common "elt"
   (:lambda (seq nth)
      (:if (common:sequencep seq)
           (:if (core:%fixnump nth)
                (:if (core:listp seq)
                     (mu:nth nth seq)
                     (mu:svref seq nth))
                (core:raise seq 'common:elt "not a sequence"))
           (core:raise nth 'common:elt "not a fixnum"))))

;;;
;;; finds
;;;
(mu:intern common "%findl"
   (:lambda (predicate seq)
     (mu:car
      ((:lambda (length)
         (mu:fix
          (:lambda (nth)
            (:if (core:listp nth)
                 nth
                 (:if (mu:eq nth length)
                      ()
                      ((:lambda (elt)
                         (:if (core:%apply predicate `(,elt))
                              `(,elt)
                              (core:1+ nth)))
                       (common:elt seq nth)))))
          0))
       (common:length seq)))))

(mu:intern common "findl-if"
   (:lambda (predicate seq)
     (:if (core:functionp predicate)
          (:if (common:sequencep seq)
               (common:%findl predicate seq)
               (core:raise seq 'common:findl-if "not a sequence"))
          (core:raise predicate 'common:findl-if "not a predicate"))))

(mu:intern common "%findr"
   (:lambda (predicate seq)
     (mu:car
      (mu:fix
       (:lambda (nth)
         (:if (core:listp nth)
              nth
              (:if (mu:eq nth 0)
                   ()
                   ((:lambda (elt)
                      (:if (core:%apply predicate `(,elt))
                           `(,elt)
                           (core:1- nth)))
                    (common:elt seq (core:1- nth))))))
       (common:length seq)))))

(mu:intern common "findr-if"
   (:lambda (predicate seq)
     (:if (core:functionp predicate)
          (:if (common:sequencep seq)
               (common:%findr predicate seq)
               (core:raise seq 'common:findr-if "not a sequence"))
          (core:raise predicate 'common:findr-if "not a predicate"))))

;;;
;;; positions
;;;
(mu:intern common "%positionl"
   (:lambda (predicate seq)
     (mu:car
      ((:lambda (length)
         (mu:fix
          (:lambda (nth)
            (:if (core:listp nth)
                 nth
                 (:if (mu:eq nth length)
                      ()
                      ((:lambda (elt)
                         (:if (core:%apply predicate `(,elt))
                              `(,nth)
                              (core:1+ nth)))
                       (common:elt seq nth)))))
          0))
       (common:length seq)))))

(mu:intern common "positionl-if"
   (:lambda (predicate seq)
     (:if (core:functionp predicate)
          (:if (common:sequencep seq)
               (common:%positionl predicate seq)
               (core:raise list 'common:positionl-if "not a sequence"))
          (core:raise predicate 'common:positionl-if "not a function"))))

(mu:intern common "%positionr"
   (:lambda (predicate seq)
     (mu:car
      (mu:fix
       (:lambda (nth)
         (:if (core:listp nth)
              nth
              (:if (mu:eq nth 0)
                   ()
                   ((:lambda (elt)
                      (:if (core:%apply predicate `(,elt))
                           `(,(core:1- nth))
                           (core:1- nth)))
                    (common:elt seq (core:1- nth))))))
       (common:length seq)))))

(mu:intern common "positionr-if"
   (:lambda (predicate seq)
      (:if (core:functionp predicate)
           (:if (common:sequencep seq)
                (common:%positionr predicate seq) 
                (core:raise seq 'common:positionr "not a sequence"))
           (core:raise predicate 'common:positionr "not a function"))))

;;;
;;; counts
;;;
(mu:intern common "%count"
   (:lambda (predicate seq)
      ((:lambda (length)
         (mu:cdr
          (mu:fix
           (:lambda (loop)
             ((:lambda (nth count)
                (:if (mu:eq nth length)
                     loop
                     ((:lambda (elt)
                         (:if (core:%apply predicate `(,elt))
                              (mu:cons (core:1+ nth) (core:1+ count))
                              (mu:cons (core:1+ nth) count)))
                        (common:elt seq nth))))
              (mu:car loop)
              (mu:cdr loop)))
             (mu:cons 0 0))))
       (common:length seq))))

(mu:intern common "count-if"
   (:lambda (predicate seq)
      (:if (core:functionp predicate)
           (:if (common:sequencep seq)
                (common:%count predicate seq) 
                (core:raise seq 'common:count-if "not a sequence"))
           (core:raise predicate 'common:count-if "not a function"))))

;;;
;;; remove
;;;
(mu:intern common "%remove"
   (:lambda (predicate seq)
      ((:lambda (length)
         (mu:cdr
          (mu:fix
           (:lambda (loop)
             ((:lambda (nth list)
                (:if (mu:eq nth length)
                     loop
                     ((:lambda (elt)
                         (:if (core:%apply predicate `(,elt))
                              (mu:cons (core:1+ nth) list)
                              (mu:cons (core:1+ nth) (mu:cons elt list))))
                        (common:elt seq nth))))
              (mu:car loop)
              (mu:cdr loop)))
             (mu:cons 0 ()))))
       (common:length seq))))

(mu:intern common "remove-if"
   (:lambda (predicate seq)
     (:if (core:functionp predicate)
          (:if (common:sequencep seq)
               (common:%remove predicate seq)
               (core:raise seq 'common:remove-if "not a sequence"))
          (core:raise predicate 'common:remove "not a function"))))

;;;
;;; copy, reverse
;;;
(mu:intern common "copy-seq"
   (:lambda (list)
     (:if (core:listp list)
          (core:%mapcar common:identity list)             
          (core:raise list 'common:copy-list "not a list"))))

(mu:intern common "%reverse"
   (:lambda (seq)
     ((:lambda (list)
        (:if (core:listp seq)
             list
             (mu:make-vector :t list)))
      ((:lambda (length)
         (mu:cdr
          (mu:fix
           (:lambda (loop)
             ((:lambda (nth list)
                (:if (mu:eq nth length)
                     loop
                     ((:lambda (elt)
                        (mu:cons (core:1+ nth) (mu:cons elt list)))
                     (common:elt seq nth))))
              (mu:car loop)
              (mu:cdr loop)))
             (mu:cons 0 ()))))
       (common:length seq)))))

(mu:intern common "reverse"
   (:lambda (seq)
     (:if (core:listp list)
          (core:%reverse list)
          (core:raise seq 'common:reverse "not a sequence"))))

;;; utilities
(mu:intern common "%coerce-seq"
   (:lambda (seq type)
     ((:lambda (seq-type)
        (:if (mu:eq seq-type 'cons)
             (:if (mu:eq type 'cons)
                  seq
                  (mu:make-vector :t seq))
             (mu:fix
              (:lambda (loop)
                ((:lambda (nth list)
                   (:if (mu:eq nth (common:length seq))
                        loop
                        (mu:cons (core:1+ nth) (mu:cons (common:elt seq nth) list))))
                 (mu:car loop)
                 (mu:cdr loop)))
              (mu:cons 0 ()))))
      (core:type-of seq))))
