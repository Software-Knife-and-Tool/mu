;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnums
;;;
(libcore:intern :prelude "1+"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (libcore:fx-add n 1)
          (prelude:raise n 'prelude:1+ "not a fixnum"))))

(libcore:intern :prelude "1-"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (libcore:fx-sub n 1)
          (prelude:raise n 'prelude:1- "not a fixnum"))))

(libcore:intern :prelude "truncate"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(libcore:fx-div n m) ,@(libcore:fx-sub n (libcore:fx-mul m (libcore:fx-div n m))))
               (prelude:raise m 'prelude:truncate "not a fixnum"))
          (prelude:raise n 'prelude:truncate "not a fixnum"))))

;;; round to negative infinity
(libcore:intern :prelude "floor"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(libcore:fx-div n m) ,@(libcore:fx-sub n (libcore:fx-mul m (libcore:fx-div n m))))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:floor "not a fixnum"))))

;;; round to positive infinity
(libcore:intern :prelude "ceiling"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(libcore:fx-div n m) ,@(libcore:fx-sub n (libcore:fx-mul m (libcore:fx-div n m))))
               (prelude:raise m 'prelude:ceiling "not a fixnum"))
          (prelude:raise n 'prelude:ceiling "not a fixnum"))))

(libcore:intern :prelude "mod"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (libcore:cdr (prelude:floor m n))
               (prelude:raise m 'prelude:mod "not a fixnum"))
          (prelude:raise n 'prelude:mod "not a fixnum"))))

(libcore:intern :prelude "rem"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (libcore:cdr (prelude:truncate m n))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:rem "not a fixnum"))))

;;; round to nearest
(libcore:intern :prelude "round"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (libcore:fx-sub (libcore:fx-div n m) (libcore:fx-sub n (libcore:fx-mul m (libcore:fx-div n m))))
               (prelude:raise m 'prelude:round "not a fixnum"))
          (prelude:raise n 'prelude:round "not a fixnum"))))
