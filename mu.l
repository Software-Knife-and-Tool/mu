(lib:intern () "list" (lambda (&rest list) list))

(lib:intern () "list*" (lambda (&rest args)
  ((lambda (loop) (lib:apply loop `(,loop ,args)))
   (lambda (loop args)
     (if (lib:cdr args)
         (lib:cons (lib:car args) (lib:apply loop `(,loop ,(lib:cdr args))))
         (lib:car args))))))

(define-macro progn (&rest body)
  (list (list* 'lambda () body)))

(define-macro when (test &rest body)
  (list 'if test (list* 'progn body)))

(define-macro unless (test &rest body)
  (list 'if test () (list* 'progn body)))
